use console::strip_ansi_codes;
use unicode_width::UnicodeWidthChar;

pub fn str_width(s: &str) -> usize {
    strip_ansi_codes(s)
        .chars()
        .flat_map(|ch| {
            if cjk::is_cjk_codepoint(ch) {
                UnicodeWidthChar::width_cjk(ch)
            } else {
                UnicodeWidthChar::width(ch)
            }
        })
        .sum()
}
