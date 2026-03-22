use crate::strip_ansi;

pub struct Buffer {
    styled: String,
    styled_lines: Vec<(usize, usize)>,
    plain: String,
    plain_lines: Vec<(usize, usize)>,
}

impl Buffer {
    pub fn line_indicies(&self, start: usize, len: usize) -> impl Iterator<Item = (usize, &str)> {
        self.styled_lines[start..start + len]
            .iter()
            .map(|&(lstart, lend)| &self.styled[lstart..lend])
            .enumerate()
    }
}

impl From<String> for Buffer {
    fn from(value: String) -> Self {
        let plain = strip_ansi(&value);
        let styled_lines = line_spans(&value);
        let plain_lines = line_spans(&plain);

        Buffer {
            styled: value,
            styled_lines,
            plain,
            plain_lines,
        }
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
