use std::io::Write;
use ansi_term::Style;
use pulldown_cmark::Alignment;
use console::{measure_text_width, strip_ansi_codes};
use crate::words::Words;

pub struct Table {
    titles: Vec<String>,
    rows: Vec<Vec<String>>,
    width: usize,
}

impl Table {
    pub fn new(titles: Vec<String>, rows: Vec<Vec<String>>, width: usize) -> Self {
        Table {
            titles,
            rows,
            width,
        }
    }

    pub fn print(self, paper_style: Style, alignment: &[Alignment]) -> String {
        let Table { titles, rows, width } = self;

        // NOTE: for now, styling is not supported within tables because that gets really hard
        let titles = titles.iter()
            .map(|title| strip_ansi_codes(title).trim().to_string())
            .collect::<Vec<_>>();
        let rows = rows.iter()
            .map(|row| row.iter()
                 .map(|cell| strip_ansi_codes(cell).trim().to_string())
                 .collect()
             )
            .collect::<Vec<Vec<_>>>();

        let num_cols = usize::max(
            titles.len(),
            rows.iter()
                .map(|row| row.len())
                .max()
                .unwrap_or(0)
        );

        let mut title_longest_words = titles.iter()
            .map(|title| Words::new(title)
                .map(|word| word.trim().len())
                .max()
                .unwrap_or(0)
            )
            .collect::<Vec<_>>();
        title_longest_words.resize(num_cols, 0);
        let longest_words = rows.iter()
            .map(|row| row
                .iter()
                .map(|cell| Words::new(cell)
                    .map(|word| word.trim().len())
                    .max()
                    .unwrap_or(0)
                )
                .collect::<Vec<_>>()
            )
            .fold(title_longest_words.clone(), |mut chars, row| {
                for i in 0..row.len() {
                    chars[i] = usize::max(chars[i], row[i]);
                }
                chars
            });

        let mut title_chars = titles.iter()
            .map(|title| title
                .lines()
                .map(measure_text_width)
                .max()
                .unwrap_or(0)
            )
            .collect::<Vec<_>>();
        title_chars.resize(num_cols, 0);
        let max_chars_per_col = rows.iter()
            .map(|row| row
                .iter()
                .map(|cell| cell
                    .lines()
                    .map(measure_text_width)
                    .max()
                    .unwrap_or(0)
                )
                .collect::<Vec<_>>()
            )
            .fold(title_chars.clone(), |mut chars, row| {
                for i in 0..row.len() {
                    chars[i] = usize::max(1, usize::max(chars[i], row[i]));
                }
                chars
            });

        let total_chars: usize = max_chars_per_col.iter().sum();
        let max_chars_width = width.saturating_sub(4 + (num_cols - 1) * 3);
        let col_widths = if total_chars < max_chars_width {
            max_chars_per_col
        } else {
            max_chars_per_col
                .into_iter()
                .enumerate()
                .map(|(i, chars)| usize::max(longest_words[i], (max_chars_width as f64 * chars as f64 / total_chars as f64) as usize))
                .collect()
        };
        if col_widths.iter().sum::<usize>() > max_chars_width {
            return format!("{}", paper_style.paint("[Table too large to fit]"));
        }

        let mut buffer = vec![];
        print_separator(&mut buffer, &col_widths, '─', '┌', '┬', '┐', paper_style);
        if !titles.is_empty() {
            print_row(&mut buffer, &col_widths, alignment, &titles, paper_style);
            print_separator(&mut buffer, &col_widths, '═', '╞', '╪', '╡', paper_style);
        }
        let row_count = rows.len();
        for (i, row) in rows.into_iter().enumerate() {
            print_row(&mut buffer, &col_widths, alignment, &row, paper_style);
            if i != row_count - 1 {
                print_separator(&mut buffer, &col_widths, '─', '├', '┼', '┤', paper_style);
            }
        }
        print_separator(&mut buffer, &col_widths, '─', '└', '┴', '┘', paper_style);

        String::from_utf8(buffer).unwrap()
    }
}

fn print_row<W: Write>(w: &mut W, cols: &[usize], alignment: &[Alignment], row: &[String], paper_style: Style) {
    let mut row_words = row
        .into_iter()
        .map(|s| Words::new(s))
        .collect::<Vec<_>>();
    loop {
        let mut done = true;
        write!(w, "{}", paper_style.paint("│")).unwrap();
        for (i, words) in row_words.iter_mut().enumerate() {
            let mut line = match words.next() {
                Some(line) => line.trim().to_string(),
                None => {
                    write!(w, "{}", paper_style.paint(format!(" {: <width$} │", " ", width=cols[i]))).unwrap();
                    continue;
                }
            };
            loop {
                match words.next() {
                    Some(next) => {
                        if measure_text_width(&line) + measure_text_width(&next) <= cols[i] {
                            line += &next;
                        } else {
                            words.undo();
                            done = false;
                            break;
                        }
                    }
                    None => break,
                };
            }
            line = line.trim().to_string();
            let padded = if alignment[i] == Alignment::Center {
                format!(" {: ^width$} │", line, width=cols[i])
            } else if alignment[i] == Alignment::Right {
                format!(" {: >width$} │", line, width=cols[i])
            } else {
                format!(" {: <width$} │", line, width=cols[i])
            };
            write!(w, "{}", paper_style.paint(padded)).unwrap();
        }
        write!(w, "\n").unwrap();
        if done {
            break;
        }
    }
}

fn print_separator<W: Write>(w: &mut W, cols: &[usize], mid: char, left: char, cross: char, right: char, paper_style: Style) {
    let line = cols.iter()
        .map(|width| mid.to_string().repeat(*width))
        .collect::<Vec<_>>()
        .join(&format!("{}{}{}", mid, cross, mid));
    write!(w, "{}\n", paper_style.paint(format!("{}{}{}{}{}", left, mid, line, mid, right))).unwrap();
}
