use crossterm::{Command, cursor};

use crate::{Pos, strip_ansi};

pub struct Buffer {
    // Current cursor position
    cpos: Pos,
    // Last rendered buffer position
    bpos: Pos,
    // Size of the current terminal
    tsize: Pos,
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
    pub fn new(styled: String, tsize: Pos) -> Buffer {
        let plain = strip_ansi(&styled);
        let styled_lines = line_spans(&styled);
        let plain_lines = line_spans(&plain);

        Buffer {
            bpos: Pos::zero(),
            cpos: Pos::zero(),
            tsize,
            styled,
            styled_lines,
            plain,
            plain_lines,
        }
    }

    pub fn set_tsize(&mut self, tsize: Pos) {
        self.tsize = tsize;
    }

    pub fn set_cursor(&mut self, cpos: Pos) {
        self.cpos = cpos;
    }

    pub fn curr_line(&self) -> usize {
        self.bpos.line
    }

    pub fn cursor_render_cmd(&self) -> impl Command {
        cursor::MoveTo(self.cpos.col as u16, self.cpos.line as u16)
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

    pub fn cursor_down(&mut self) -> bool {
        if self.cpos.line + 1 < self.tsize.line {
            self.cpos = self.cpos.down();
        } else if self.bpos.line < self.lines() {
            return true;
        }
        false
    }

    pub fn cursor_up(&mut self) -> bool {
        if self.cpos.line != 0 {
            self.cpos = self.cpos.up();
        } else if self.bpos.line > self.tsize.line {
            return true;
        }
        false
    }

    pub fn cursor_left(&mut self) {
        self.cpos = self.cpos.left();
    }

    pub fn cursor_right(&mut self) {
        self.cpos = self.cpos.right();
        self.cpos.col = self.cpos.col.min(self.tsize.col - 1);
    }

    pub fn cursor_start_line(&mut self) {
        self.cpos.col = 0;
    }

    pub fn cursor_end_line(&mut self) {
        self.cpos.col = self.tsize.col;
    }

    pub fn cursor_high(&mut self) {
        self.cpos.line = 0;
    }

    pub fn cursor_mid(&mut self) {
        self.cpos.line = (self.tsize.line - 1) / 2;
    }

    pub fn cursor_low(&mut self) {
        self.cpos.line = self.tsize.line - 1;
    }

    pub fn cursor_home(&mut self) {
        self.cpos = Pos::zero();
    }

    pub fn cursor_word_right(&mut self) -> usize {
        todo!()
    }

    pub fn scroll_up(&mut self, scroll: usize) {
        self.bpos.line = self.bpos.line.saturating_add(scroll);
    }

    pub fn scroll_down(&mut self, scroll: usize) {
        self.bpos.line = self.bpos.line.saturating_sub(scroll);
    }

    pub fn scroll_to(&mut self, line: usize) {
        self.bpos.line = line;
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
