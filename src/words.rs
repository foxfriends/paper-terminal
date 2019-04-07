pub struct Words<S: AsRef<str>> {
    source: S,
    position: usize,
    previous: usize,
}

impl<S: AsRef<str>> Words<S> {
    pub fn new(source: S) -> Self {
        Self {
            source,
            previous: 0,
            position: 0,
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
                return None
            } else {
                return Some(" ".to_string())
            }
        }
        let mut len = 0;
        while start+len < chars.len() {
            if chars[start+len] == '-' {
                len += 1;
                break;
            }
            if chars[start+len].is_whitespace() {
                break;
            }
            len += 1;
        }
        self.position += len;
        if chars[0].is_whitespace() {
            return Some(String::from(" ") + &chars[start..start+len].iter().collect::<String>())
        } else {
            return Some(chars[start..start+len].iter().collect::<String>())
        }
    }
}
