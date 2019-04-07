use std::path::PathBuf;
use std::io;
use std::fs;
use structopt::StructOpt;
use terminal_size::{Width, terminal_size};
use pulldown_cmark::{Parser, Options};
use ansi_term::{Style, Colour};

mod printer;
mod words;
mod table;
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

    /// Don't bother with the whole paper part, just print the markdown nicely
    #[structopt(short, long)]
    pub no_paper: bool,

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
    let paper_style = Colour::Black.on(Colour::White);
    let shadow_style = Style::default().on(Colour::Fixed(8));
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
        let parser = Parser::new_ext(&source, Options::all());
        if opts.dev {
            for event in parser {
                println!("{:?}", event);
            }
        } else if opts.no_paper {
            let mut printer = Printer::new("", "", "", width, Style::default(), &opts);
            for event in parser {
                printer.handle(event);
            }
        } else {
            println!("{}{}", centering, blank_line);
            for _ in 0..v_margin {
                println!("{}{}{}", centering, blank_line, end_shadow);
            }

            let mut printer = Printer::new(&centering, &margin, &end_shadow, width - 2 * h_margin, paper_style, &opts);
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
        let stdin = io::stdin();
        loop {
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => return,
                Ok(..) => print!("{}", line),
                Err(error) => {
                    eprintln!("{}", error);
                    return
                }
            }
        }
    } else {
        let sources = opts.files.clone()
            .into_iter()
            .map(|path| fs::read_to_string(&path),
        );
        print(opts, sources);
    }
}
