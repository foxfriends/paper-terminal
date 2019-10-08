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
            len += 1;
        }
        self.position += len;
        if chars[0].is_whitespace() {
            if self.preserve_whitespace {
                Some(chars[0..start + len].into_iter().collect::<String>())
            } else {
                Some(
                    String::from(" ") + &chars[start..start + len].into_iter().collect::<String>(),
                )
            }
        } else {
            Some(chars[start..start + len].into_iter().collect::<String>())
        }
    }
}
