use crate::{Pos, strip_ansi};

pub struct Buffer {
    // Last rendered buffer position
    pub(crate) bpos: Pos,
    // Current cursor position
    pub(crate) cpos: Pos,
    // Style contents to be paged
    styled: String,
    // Line spans of paged contents
    styled_lines: Vec<(usize, usize)>,
    // Plain text stripped of ansi escape codes
    plain: String,
    // Line spans of plain text
    plain_lines: Vec<(usize, usize)>,
}

impl Buffer {
    pub fn new(styled: String) -> Buffer {
        let plain = strip_ansi(&styled);
        let styled_lines = line_spans(&styled);
        let plain_lines = line_spans(&plain);

        Buffer {
            bpos: Pos::zero(),
            cpos: Pos::zero(),
            styled,
            styled_lines,
            plain,
            plain_lines,
        }
    }

    pub fn lines(&self) -> usize {
        self.styled_lines.len()
    }

    pub fn line_indicies(&self, start: usize, len: usize) -> impl Iterator<Item = (usize, &str)> {
        self.styled_lines[start..start + len]
            .iter()
            .map(|&(lstart, lend)| &self.styled[lstart..lend])
            .enumerate()
    }

    pub fn scroll_up(&mut self, scroll: usize) {
        self.bpos.line = self.bpos.line.saturating_add(scroll);
    }

    pub fn scroll_down(&mut self, scroll: usize) {
        self.bpos.line = self.bpos.line.saturating_sub(scroll);
    }

    pub fn scroll_to(&mut self, line: usize) {
        self.bpos.line = line
    }

    pub fn word_right_from(&self, line: usize, col: usize) -> usize {
        todo!()
    }
}

fn line_spans(buf: &str) -> Vec<(usize, usize)> {
    buf.lines()
        .map(str::len)
        .scan(0, |acc, x| {
            let ret = Some((*acc, *acc + x));
            *acc += x + 1;
            ret
        })
        .collect()
}
