use cjk::is_cjk_codepoint;

pub struct Words<S: AsRef<str>> {
    source: S,
    position: usize,
    previous: usize,
    preserve_whitespace: bool,
}

impl<S: AsRef<str>> Words<S> {
    pub fn new(source: S) -> Self {
        Self {
            source,
            previous: 0,
            position: 0,
            preserve_whitespace: false,
        }
    }

    pub fn preserving_whitespace(source: S) -> Self {
        Self {
            source,
            previous: 0,
            position: 0,
            preserve_whitespace: true,
        }
    }
}

impl<S: AsRef<str>> Words<S> {
    pub fn undo(&mut self) {
        self.position = self.previous;
    }
}

// NOTE: this almost certainly does some extra processing... but for my sanity,
// we accept that
fn may_end_word_cjk(ch: char) -> bool {
    // simplified chinese
    !"$(£¥·'\"〈《「『【〔〖〝﹙﹛＄（．［｛￡￥"
        .chars()
        .any(|c| c == ch)
    // traditional chinese
    && !"([{£¥'\"‵〈《「『〔〝︴﹙﹛（｛︵︷︹︻︽︿﹁﹃﹏"
        .chars()
        .any(|c| c == ch)
    // japanese
    && !"([｛〔〈《「『【〘〖〝'\"｟«"
        .chars()
        .any(|c| c == ch)
    // japanese inseparable
    && !"—...‥〳〴〵"
        .chars()
        .any(|c| c == ch)
    // korean
    && !"$([\\{£¥'\"々〇〉》」〔＄（［｛｠￥￦ #"
        .chars()
        .any(|c| c == ch)
}

fn may_start_word_cjk(ch: char) -> bool {
    // simplified chinese
    !"!%),.:;?]}¢°·'\"†‡›℃∶、。〃〆〕〗〞﹚﹜！＂％＇），．：；？！］｝～"
        .chars()
        .any(|c| c == ch)
    // traditional chinese
    && !"!),.:;?]}¢·–— '\"• 、。〆〞〕〉》」︰︱︲︳﹐﹑﹒﹓﹔﹕﹖﹘﹚﹜！），．：；？︶︸︺︼︾﹀﹂﹗］｜｝､"
        .chars()
        .any(|c| c == ch)
    // japenese
    && !")]｝〕〉》」』】〙〗〟'\"｠»"
        .chars()
        .any(|c| c == ch)
    && !"ヽヾーァィゥェォッャュョヮヵヶぁぃぅぇぉっゃゅょゎゕゖㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿ々〻"
        .chars()
        .any(|c| c == ch)
    && !"‐゠–〜? ! ‼ ⁇ ⁈ ⁉・、:;,。."
        .chars()
        .any(|c| c == ch)
    // japanese inseparable
    && !"—...‥〳〴〵"
        .chars()
        .any(|c| c == ch)
    // korean
    && !"!%),.:;?]}¢°'\"†‡℃〆〈《「『〕！％），．：；？］｝"
        .chars()
        .any(|c| c == ch)
}

impl<S: AsRef<str>> Iterator for Words<S> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.previous = self.position;
        let chars: Vec<char> = self.source.as_ref().chars().skip(self.position).collect();
        let mut start = 0;
        while start < chars.len() && chars[start].is_whitespace() {
            start += 1;
        }
        self.position += start;
        if start == chars.len() {
            if chars.len() == 0 {
                return None;
            } else if self.preserve_whitespace {
                return Some(chars[..].into_iter().collect());
            } else {
                return Some(" ".to_string());
            }
        }
        let mut len = 0;
        while start + len < chars.len() {
            if chars[start + len] == '-' {
                len += 1;
                break;
            }
            if chars[start + len].is_whitespace() {
                break;
            }
            if len != 0
                // Before or after cjk characters, we can usually break line, unless it's one of the exceptions.
                // I got the exceptions off Wikipedia:
                //     https://en.wikipedia.org/wiki/Line_breaking_rules_in_East_Asian_languages
                && (is_cjk_codepoint(chars[start + len - 1]) || is_cjk_codepoint(chars[start + len]))
                && may_end_word_cjk(chars[start + len - 1])
                && may_start_word_cjk(chars[start + len])
            {
                break;
            }
            len += 1;
        }
        self.position += len;
        if chars[0].is_whitespace() {
            if self.preserve_whitespace {
                return Some(chars[0..start + len].into_iter().collect::<String>());
            } else {
                return Some(
                    String::from(" ") + &chars[start..start + len].into_iter().collect::<String>(),
                );
            }
        } else {
            return Some(chars[start..start + len].into_iter().collect::<String>());
        }
    }
}
