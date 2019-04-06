pub struct Words<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Words<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let chars: Vec<char> = self.source.chars().skip(self.position).collect();
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
