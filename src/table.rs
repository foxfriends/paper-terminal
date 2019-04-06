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
            .map(|title| strip_ansi_codes(title))
            .collect::<Vec<_>>();
        let rows = rows.iter()
            .map(|row| row.iter()
                 .map(|cell| strip_ansi_codes(cell))
                 .collect()
             )
            .collect::<Vec<Vec<_>>>();

        //let format = FormatBuilder::new()
            //.borders('│')
            //.column_separator('│')
            //.separator(LinePosition::Top, LineSeparator::new('─', '┬', '┌', '┐'))
            //.separator(LinePosition::Title, LineSeparator::new('═', '╪', '╞', '╡'))
            //.separator(LinePosition::Intern, LineSeparator::new('─', '┼', '├', '┤'))
            //.separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '└', '┘'))
            //.build();

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
                    chars[i] = usize::max(chars[i], row[i]);
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
        "".to_string()
    }
}
/*
        let mut buffer = vec![];
        table.print(&mut buffer).unwrap();
        let content = String::from_utf8(buffer).unwrap();
        for line in content.lines() {
            let (prefix, prefix_len) = self.prefix();
            let (suffix, suffix_len) = self.suffix();
            let available_width = self.width - prefix_len - suffix_len;
            let used_width = usize::min(available_width, line.chars().count());
            println!(
                "{}{}{}{}{}{}{}{}",
                self.centering,
                self.margin,
                prefix,
                self.paper_style.paint(line.chars().take(available_width).collect::<String>()),
                self.paper_style.paint(" ".repeat(available_width - used_width)),
                suffix,
                self.margin,
                self.shadow,
                );
        }
    }
}
*/
