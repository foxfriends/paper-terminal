use ansi_term::Style;
use clap::{CommandFactory, Parser as _};
use clap_complete::Shell;
use console::strip_ansi_codes;
use pulldown_cmark::{Options, Parser};
use std::convert::TryInto;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use syncat_stylesheet::Stylesheet;
use terminal_size::{terminal_size, Width};

mod dirs;
mod printer;
mod str_width;
mod table;
mod termpix;
mod words;

use printer::Printer;
use str_width::str_width;
use words::Words;

/// Prints papers in your terminal
#[derive(clap::Parser, Debug)]
#[clap(name = "paper")]
#[clap(rename_all = "kebab-case")]
pub struct Opts {
    /// Margin (shortcut for horizontal and vertical margin set to the same value)
    #[structopt(short = 'm', long, default_value = "6")]
    pub margin: usize,

    /// Horizontal margin (overrides --margin)
    #[structopt(long)]
    pub h_margin: Option<usize>,

    /// Vertical margin (overrides --margin)
    #[structopt(long)]
    pub v_margin: Option<usize>,

    /// The width of the paper (including the space used for the margin)
    #[structopt(short = 'w', long, default_value = "92")]
    pub width: usize,

    /// Don't parse as Markdown, just render the plain text on a paper
    #[structopt(short = 'p', long)]
    pub plain: bool,

    /// The length to consider tabs as.
    #[structopt(short, long, default_value = "4")]
    pub tab_length: usize,

    /// Hide link URLs
    #[structopt(short = 'U', long)]
    pub hide_urls: bool,

    /// Disable drawing images
    #[structopt(short = 'I', long)]
    pub no_images: bool,

    /// Position paper on the left edge of the terminal, instead of centred.
    #[structopt(short = 'l', long)]
    pub left: bool,

    /// Position paper on the right edge of the terminal, instead of centred.
    #[structopt(short = 'r', long)]
    pub right: bool,

    /// Use syncat to highlight code blocks. Requires you have syncat installed.
    #[structopt(short, long)]
    pub syncat: bool,

    /// Print in debug mode
    #[structopt(long)]
    pub dev: bool,

    /// Files to print
    #[structopt(name = "FILE")]
    pub files: Vec<PathBuf>,

    /// Generate shell completions
    #[structopt(long)]
    completions: Option<Shell>,
}

fn normalize(tab_len: usize, source: &str) -> String {
    source
        .lines()
        .map(|line| {
            let mut len = 0;
            let line = strip_ansi_codes(line);
            if line.contains('\t') {
                line.chars()
                    .flat_map(|ch| {
                        if ch == '\t' {
                            let missing = tab_len - (len % tab_len);
                            len += missing;
                            vec![' '; missing]
                        } else {
                            len += 1;
                            vec![ch]
                        }
                    })
                    .collect::<String>()
                    .into()
            } else {
                line
            }
        })
        .map(|line| format!("{}\n", line))
        .collect::<String>()
}

fn print<I>(opts: Opts, sources: I)
where
    I: Iterator<Item = Result<String, std::io::Error>>,
{
    let h_margin = opts.h_margin.unwrap_or(opts.margin);
    let v_margin = opts.v_margin.unwrap_or(opts.margin);
    let terminal_width = terminal_size()
        .map(|(Width(width), _)| width)
        .unwrap_or(opts.width as u16) as usize;
    let width = usize::min(opts.width, terminal_width - 1);

    if width < h_margin * 2 + 40 {
        eprintln!("The width is too short!");
        return;
    }

    let left_space = match (opts.left, opts.right) {
        (true, false) => "".to_owned(),
        (false, true) => " ".repeat(terminal_width.saturating_sub(width) - 1),
        _ => " ".repeat((terminal_width.saturating_sub(width)) / 2),
    };

    let stylesheet = Stylesheet::from_file(dirs::active_color().join("paper.syncat"))
        .unwrap_or_else(|_| {
            include_str!("default.syncat")
                .parse::<Stylesheet>()
                .unwrap()
        });
    let paper_style: Style = stylesheet
        .style(&"paper".into())
        .unwrap_or_default()
        .try_into()
        .unwrap_or_default();
    let shadow_style: Style = stylesheet
        .style(&"shadow".into())
        .unwrap_or_default()
        .try_into()
        .unwrap_or_default();
    let blank_line = format!("{}", paper_style.paint(" ".repeat(width)));
    let end_shadow = format!("{}", shadow_style.paint(" "));
    let margin = format!("{}", paper_style.paint(" ".repeat(h_margin)));
    let available_width = width - 2 * h_margin;
    for source in sources {
        let source = match source {
            Ok(source) => normalize(opts.tab_length, &source),
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };
        if opts.plain {
            println!("{}{}", left_space, blank_line);
            for _ in 0..v_margin {
                println!("{}{}{}", left_space, blank_line, end_shadow);
            }

            for line in source.lines() {
                let mut buffer = String::new();
                let mut indent = None;
                for word in Words::preserving_whitespace(line) {
                    if str_width(&buffer) + str_width(&word) > available_width {
                        println!(
                            "{}{}{}{}{}{}",
                            left_space,
                            margin,
                            paper_style.paint(&buffer),
                            paper_style.paint(
                                " ".repeat(available_width.saturating_sub(str_width(&buffer)))
                            ),
                            margin,
                            shadow_style.paint(" "),
                        );
                        buffer.clear();
                    }
                    if buffer.is_empty() {
                        if indent.is_none() {
                            let indent_len =
                                word.chars().take_while(|ch| ch.is_whitespace()).count();
                            indent = Some(word[0..indent_len].to_string());
                        }
                        buffer.push_str(indent.as_ref().unwrap());
                        buffer.push_str(word.trim());
                    } else {
                        buffer.push_str(&word);
                    }
                }
                println!(
                    "{}{}{}{}{}{}",
                    left_space,
                    margin,
                    paper_style.paint(&buffer),
                    paper_style
                        .paint(" ".repeat(available_width.saturating_sub(str_width(&buffer)))),
                    margin,
                    shadow_style.paint(" "),
                );
            }
            for _ in 0..v_margin {
                println!("{}{}{}", left_space, blank_line, end_shadow);
            }
            println!("{} {}", left_space, shadow_style.paint(" ".repeat(width)));
        } else if opts.dev {
            let parser = Parser::new_ext(&source, Options::all());
            for event in parser {
                println!("{:?}", event);
            }
        } else {
            let parser = Parser::new_ext(&source, Options::all());
            println!("{}{}", left_space, blank_line);
            for _ in 0..v_margin {
                println!("{}{}{}", left_space, blank_line, end_shadow);
            }

            let mut printer =
                Printer::new(&left_space, &margin, available_width, &stylesheet, &opts);
            for event in parser {
                printer.handle(event);
            }

            for _ in 0..v_margin {
                println!("{}{}{}", left_space, blank_line, end_shadow);
            }
            println!("{} {}", left_space, shadow_style.paint(" ".repeat(width)));
        }
    }
}

fn main() {
    let opts = Opts::parse();

    if opts.completions.is_some() {
        let shell = opts.completions.or_else(Shell::from_env).unwrap();
        let mut opts = Opts::command();
        let name = opts.get_name().to_string();
        clap_complete::generate(shell, &mut opts, name, &mut std::io::stdout());
        std::process::exit(0);
    }

    if opts.files.is_empty() {
        let mut string = String::new();
        io::stdin().read_to_string(&mut string).unwrap();
        print(opts, vec![Ok(string)].into_iter());
    } else {
        let sources = opts
            .files
            .clone()
            .into_iter()
            .map(|path| fs::read_to_string(&path));
        print(opts, sources);
    }
}
