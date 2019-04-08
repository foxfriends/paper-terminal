use std::path::PathBuf;
use std::io::{self, Read};
use std::fs::{self, File};
use structopt::StructOpt;
use terminal_size::{Width, terminal_size};
use pulldown_cmark::{Parser, Options};
use syncat_stylesheet::Stylesheet;

mod dirs;
mod printer;
mod table;
mod words;

use printer::Printer;

/// Prints papers in your terminal
#[derive(StructOpt, Debug)]
#[structopt(name = "paper")]
#[structopt(rename_all = "kebab-case")]
pub struct Opts {
    /// Margin (shortcut for horizontal and vertical margin the same)
    #[structopt(short, long, default_value="6")]
    pub margin: usize,

    /// Horizontal margin
    #[structopt(short, long)]
    pub h_margin: Option<usize>,

    /// Vertical margin
    #[structopt(short, long)]
    pub v_margin: Option<usize>,

    /// The width of the paper (text and margin)
    #[structopt(short, long, default_value="92")]
    pub width: usize,

    /// Don't parse as Markdown, just render the plain text on a paper
    #[structopt(short, long)]
    pub plain: bool,

    /// Use syncat to highlight code blocks. Requires you have syncat installed.
    #[structopt(short, long)]
    pub syncat: bool,

    /// Print in debug mode
    #[structopt(long)]
    pub dev: bool,

    /// Files to print
    #[structopt(name="FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

fn print<I>(opts: Opts, sources: I) where I: Iterator<Item=Result<String, std::io::Error>> {
    let h_margin = opts.h_margin.unwrap_or(opts.margin);
    let v_margin = opts.v_margin.unwrap_or(opts.margin);
    let terminal_width = terminal_size().map(|(Width(width), _)| width).unwrap_or(opts.width as u16) as usize;
    let width = usize::min(opts.width, terminal_width - 1);

    if width < h_margin * 2 + 40 {
        eprintln!("The width is too short!");
        return;
    }

    let centering = " ".repeat((terminal_width - width) / 2);

    let stylesheet = File::open(dirs::syncat_config().join("style/active/md.syncat"))
        .map_err(Into::into)
        .and_then(|mut file| Stylesheet::from_reader(&mut file))
        .unwrap_or_else(|_| include_str!("default.syncat").parse::<Stylesheet>().unwrap());
    let paper_style = stylesheet.resolve_basic(&["paper"], None).build();
    let shadow_style = stylesheet.resolve_basic(&["shadow"], None).build();
    let blank_line = format!("{}", paper_style.paint(" ".repeat(width)));
    let end_shadow = format!("{}", shadow_style.paint(" "));
    let margin = format!("{}", paper_style.paint(" ".repeat(h_margin)));
    for source in sources {
        let source = match source {
            Ok(source) => source,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };
        if opts.plain {
            println!("{}{}", centering, blank_line);
            for _ in 0..v_margin {
                println!("{}{}{}", centering, blank_line, end_shadow);
            }

            for line in source.lines() {
                println!(
                    "{}{}{}{}{}{}",
                    centering,
                    margin,
                    paper_style.paint(line),
                    paper_style.paint(" ".repeat((width - 2 * h_margin).saturating_sub(line.chars().count()))),
                    margin,
                    end_shadow,
                );
            }

            for _ in 0..v_margin {
                println!("{}{}{}", centering, blank_line, end_shadow);
            }
            println!("{} {}", centering, shadow_style.paint(" ".repeat(width)));
        } else if opts.dev {
            let parser = Parser::new_ext(&source, Options::all());
            for event in parser {
                println!("{:?}", event);
            }
        } else {
            let parser = Parser::new_ext(&source, Options::all());
            println!("{}{}", centering, blank_line);
            for _ in 0..v_margin {
                println!("{}{}{}", centering, blank_line, end_shadow);
            }

            let mut printer = Printer::new(&centering, &margin, width - 2 * h_margin, &stylesheet, &opts);
            for event in parser {
                printer.handle(event);
            }

            for _ in 0..v_margin {
                println!("{}{}{}", centering, blank_line, end_shadow);
            }
            println!("{} {}", centering, shadow_style.paint(" ".repeat(width)));
        }
    }
}

fn main() {
    let opts = Opts::from_args();

    if opts.files.is_empty() {
        let mut string = String::new();
        io::stdin().read_to_string(&mut string).unwrap();
        print(opts, vec![Ok(string)].into_iter());
    } else {
        let sources = opts.files.clone()
            .into_iter()
            .map(|path| fs::read_to_string(&path),
        );
        print(opts, sources);
    }
}
