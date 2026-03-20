use std::str::Chars;

// Inspired from: https://github.com/sharkdp/bat/blob/master/src/vscreen.rs#L349-L359
struct PlainTextIterator<'a> {
    chars: Chars<'a>,
}

impl<'a> PlainTextIterator<'a> {
    fn new(text: &'a str) -> PlainTextIterator<'a> {
        PlainTextIterator {
            chars: text.chars(),
        }
    }

    fn skip_sequence(&mut self) {
        match self.chars.next() {
            Some(']') => self.skip_osc(),
            Some('[') => self.skip_csi(),
            Some('\x20'..='\x2F') => self.skip_nf(),
            Some(_) => unreachable!(),
            None => {}
        }
    }

    fn skip_osc(&mut self) {
        #[allow(clippy::upper_case_acronyms)]
        enum ST {
            BEL,
            ESC,
        }
        let mut st = ST::BEL;
        for c in self.chars.by_ref() {
            st = match (st, c) {
                (ST::BEL, '\x07') => break,
                (ST::BEL, '\x1B') => ST::ESC,
                (ST::BEL, _) => ST::BEL,
                (ST::ESC, '\x5C') => break,
                (ST::ESC, _) => ST::BEL,
            }
        }
    }

    fn skip_csi(&mut self) {
        self.chars
            .find(|&c| !matches!(c, '\x30'..='\x3F' | '\x20'..='\x2F'));
    }

    fn skip_nf(&mut self) {
        self.chars.find(|&c| !matches!(c, '\x20'..='\x2F'));
    }
}

impl Iterator for PlainTextIterator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some('\x1B') => {
                    self.skip_sequence();
                    continue;
                }
                Some(c) => return Some(c),
                None => return None,
            }
        }
    }
}

// Strips ANSI escape sequences from the input.
pub fn strip_ansi(line: &str) -> String {
    PlainTextIterator::new(line).collect()
}
