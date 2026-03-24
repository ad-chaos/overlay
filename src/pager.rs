use std::io::{self, Stdout, Write};

use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{
    BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate, EnterAlternateScreen,
    LeaveAlternateScreen, ScrollDown, ScrollUp, disable_raw_mode, enable_raw_mode, size,
};
use crossterm::{QueueableCommand, cursor::MoveTo, execute, queue};

use crate::{Buffer, Pos};

pub struct Pager {
    // Contents to be paged
    buffer: Buffer,
    // Terminal size (cols, rows)
    tsize: Pos,
    // Stdout attached to this process
    stdout: Stdout,
    // Active VimMode
    mode: VimMode,
}

#[derive(Debug, Clone, Copy)]
enum VimMode {
    Normal,
    WaitG,
    Quit,
}

impl VimMode {
    fn quit(&self) -> bool {
        matches!(self, VimMode::Quit)
    }
}

impl Pager {
    pub fn new(buffer: String, stdout: Stdout) -> Pager {
        let (columns, rows) = size().expect("couldn't get terminal size");
        let tsize = Pos::new(columns as usize, rows as usize);
        let buffer = Buffer::new(buffer, tsize);
        Pager {
            buffer,
            stdout,
            tsize,
            mode: VimMode::Normal,
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        queue!(
            self.stdout,
            BeginSynchronizedUpdate,
            EnterAlternateScreen,
            Clear(ClearType::All),
            MoveTo(0, 0),
        )?;

        self.render_first()?;
        self.stdout.queue(EndSynchronizedUpdate)?.flush()
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.mode.quit() {
            self.stdout.queue(BeginSynchronizedUpdate)?;

            match event::read()? {
                Event::Key(key) => self.handle_key(key)?,
                Event::Resize(nc, nr) => {
                    self.tsize = Pos::new(nr as usize, nc as usize);
                    self.buffer.set_tsize(self.tsize);
                }
                _ => {}
            }

            self.stdout
                .queue(self.buffer.cursor_render_cmd())?
                .queue(EndSynchronizedUpdate)?;
            self.stdout.flush()?;
        }

        Ok(())
    }

    pub fn start(&mut self) -> io::Result<()> {
        self.init()?;
        self.run()
    }

    fn paint_lines(&mut self, buf_start: usize, cur_start: usize, len: usize) -> io::Result<()> {
        for (i, line) in self.buffer.line_indicies(buf_start, len) {
            self.stdout
                .queue(MoveTo(0, (i + cur_start) as u16))?
                .queue(Clear(ClearType::CurrentLine))?
                .queue(Print(line))?;
        }
        self.buffer
            .set_cursor(cursor::position().map(|(x, y)| Pos::new(x as usize, y as usize))?);
        Ok(())
    }

    fn render_scroll_up(&mut self, scroll: usize) -> io::Result<()> {
        let (buf_start, cur_start) = (self.buffer.curr_line(), self.tsize.line - scroll);

        self.stdout.queue(ScrollUp(scroll as u16))?;
        self.paint_lines(buf_start, cur_start, scroll)?;
        self.buffer.scroll_up(scroll);

        Ok(())
    }

    fn render_scroll_down(&mut self, scroll: usize) -> io::Result<()> {
        let (buf_start, cur_start) = (self.buffer.curr_line() - scroll - self.tsize.line, 0);

        self.stdout.queue(ScrollDown(scroll as u16))?;
        self.paint_lines(buf_start, cur_start, scroll)?;
        self.buffer.scroll_down(scroll);

        Ok(())
    }

    fn render_first(&mut self) -> io::Result<()> {
        self.buffer.scroll_to(self.tsize.line);
        self.paint_lines(0, 0, self.tsize.line)
    }

    fn render_last(&mut self) -> io::Result<()> {
        self.buffer.scroll_to(self.buffer.lines());
        self.paint_lines(self.buffer.lines() - self.tsize.line, 0, self.tsize.line)
    }

    fn handle_normal(&mut self, key: KeyEvent) -> io::Result<()> {
        use VimMode::*;
        self.mode = match key.code {
            KeyCode::Char('q') => Quit,
            KeyCode::Char('j') => {
                if self.buffer.cursor_down() {
                    self.render_scroll_up(1)?;
                }
                Normal
            }
            KeyCode::Char('k') => {
                if self.buffer.cursor_up() {
                    self.render_scroll_down(1)?;
                }
                Normal
            }
            KeyCode::Char('h') => {
                self.buffer.cursor_left();
                Normal
            }
            KeyCode::Char('l') => {
                self.buffer.cursor_right();
                Normal
            }
            KeyCode::Char('G') => {
                self.render_last()?;
                Normal
            }
            KeyCode::Char('0') => {
                self.buffer.cursor_start_line();
                Normal
            }
            KeyCode::Char('$') => {
                self.buffer.cursor_end_line();
                Normal
            }
            KeyCode::Char('H') => {
                self.buffer.cursor_high();
                Normal
            }
            KeyCode::Char('L') => {
                self.buffer.cursor_low();
                Normal
            }
            KeyCode::Char('M') => {
                self.buffer.cursor_mid();
                Normal
            }
            KeyCode::Char('g') => WaitG,
            KeyCode::Char('w') => Normal,
            _ => self.mode,
        };

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.mode {
            VimMode::Normal => self.handle_normal(key)?,
            VimMode::WaitG => {
                if let KeyCode::Char('g') = key.code {
                    self.render_first()?;
                    self.buffer.cursor_home();
                    self.mode = VimMode::Normal
                }
            }
            _ => {}
        };

        Ok(())
    }

    pub fn end(&mut self) -> io::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)?;
        disable_raw_mode()
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        self.end().unwrap();
    }
}
