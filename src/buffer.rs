use crate::{Pos, strip_ansi};

pub struct Buffer {
    styled: String,
    styled_lines: Vec<(usize, usize)>,
    plain: String,
    plain_lines: Vec<(usize, usize)>,
}

impl Buffer {
    pub fn new(styled: String) -> Buffer {
        let plain = strip_ansi(&styled);
        let styled_lines = line_spans(&styled);
        let plain_lines = line_spans(&plain);

        Buffer {
            styled,
            styled_lines,
            plain,
            plain_lines,
        }
    }

    pub fn line_indicies(&self, start: usize, len: usize) -> impl Iterator<Item = (usize, &str)> {
        self.styled_lines[start..start + len]
            .iter()
            .map(|&(lstart, lend)| &self.styled[lstart..lend])
            .enumerate()
    }

    pub fn word_right_from(&self, from: Pos) -> Pos {
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
