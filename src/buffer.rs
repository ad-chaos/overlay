use crate::strip_ansi;

pub struct Buffer {
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

    pub fn word_right_from(&self, line: usize, col: usize) -> u32 {
        let (lstart, lend) = self.plain_lines[line];
        let bline = &self.plain[lstart..lend];

        // Find the first non 'isk' character
        let Some(nisk) =
            bline[col..].find(|c| !matches!(c, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))
        else {
            return col as u32;
        };

        // // Find the next 'isk' character
        let ncol = bline[col..][nisk..]
            .find(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))
            .unwrap_or(0);

        (col + nisk + ncol) as u32
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
