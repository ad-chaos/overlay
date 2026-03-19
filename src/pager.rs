use std::io::{self, Stdout, Write};

use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{
    BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate, EnterAlternateScreen,
    LeaveAlternateScreen, ScrollDown, ScrollUp, disable_raw_mode, enable_raw_mode, size,
};
use crossterm::{QueueableCommand, cursor::MoveTo, execute, queue};

use crate::vec2::Vec2;

pub struct Pager {
    // Contains the buffer for viewing
    buffer: String,
    // Number of lines within the buffer
    lines: u32,
    // Terminal size (cols, rows)
    tsize: Vec2,
    // Last rendered line's position in the buffer
    vpos: u32,
    // Cursor Position
    cpos: Vec2,
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
        let lines = buffer.lines().count() as u32;
        let (columns, rows) = size().expect("couldn't get terminal size");
        let tsize = Vec2::new(columns as u32, rows as u32);
        Pager {
            buffer,
            lines,
            stdout,
            tsize,
            vpos: 0,
            cpos: Vec2::zero(),
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
                    self.tsize = Vec2::new(nr as u32, nc as u32);
                }
                _ => {}
            }

            self.stdout
                .queue(MoveTo(self.cpos.x as u16, self.cpos.y as u16))?
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
        for (i, line) in self.buffer.lines().skip(buf_start).take(len).enumerate() {
            self.stdout
                .queue(MoveTo(0, (i + cur_start) as u16))?
                .queue(Clear(ClearType::CurrentLine))?
                .queue(Print(line))?;
        }
        self.cpos = cursor::position().map(|(x, y)| Vec2::new(x as u32, y as u32))?;
        Ok(())
    }

    fn render_scroll_up(&mut self, scroll: u32) -> io::Result<()> {
        let (buf_start, cur_start) = (self.vpos, self.tsize.y - scroll);

        self.stdout.queue(ScrollUp(scroll as u16))?;
        self.paint_lines(buf_start as usize, cur_start as usize, scroll as usize)?;
        self.vpos = self.vpos.saturating_add(scroll);

        Ok(())
    }

    fn render_scroll_down(&mut self, scroll: u32) -> io::Result<()> {
        let (buf_start, cur_start) = (self.vpos - scroll - self.tsize.y, 0);

        self.stdout.queue(ScrollDown(scroll as u16))?;
        self.paint_lines(buf_start as usize, cur_start as usize, scroll as usize)?;
        self.vpos = self.vpos.saturating_sub(scroll);

        Ok(())
    }

    fn render_first(&mut self) -> io::Result<()> {
        self.vpos = self.tsize.y;
        self.paint_lines(0, 0, self.tsize.y as usize)
    }

    fn render_last(&mut self) -> io::Result<()> {
        self.vpos = self.lines;
        self.paint_lines(
            (self.lines - self.tsize.y) as usize,
            0,
            self.tsize.y as usize,
        )
    }

    fn handle_normal(&mut self, key: KeyEvent) -> io::Result<()> {
        use VimMode::*;
        self.mode = match key.code {
            KeyCode::Char('q') => Quit,
            KeyCode::Char('j') => {
                if self.cpos.y + 1 < self.tsize.y {
                    self.cpos = self.cpos.down();
                } else if self.vpos < self.lines {
                    self.render_scroll_up(1)?;
                }
                Normal
            }
            KeyCode::Char('k') => {
                if self.cpos.y != 0 {
                    self.cpos = self.cpos.up();
                } else if self.vpos > self.tsize.y {
                    self.render_scroll_down(1)?;
                }
                Normal
            }
            KeyCode::Char('h') => {
                self.cpos = self.cpos.left();
                Normal
            }
            KeyCode::Char('l') => {
                self.cpos = self.cpos.right();
                self.cpos.x = self.cpos.x.min(self.tsize.x - 1);
                Normal
            }
            KeyCode::Char('G') => {
                self.render_last()?;
                Normal
            }
            KeyCode::Char('0') => {
                self.cpos.x = 0;
                Normal
            }
            KeyCode::Char('$') => {
                self.cpos.x = self.tsize.x;
                Normal
            }
            KeyCode::Char('H') => {
                self.cpos.y = 0;
                Normal
            }
            KeyCode::Char('L') => {
                self.cpos.y = self.tsize.y - 1;
                Normal
            }
            KeyCode::Char('M') => {
                self.cpos.y = (self.tsize.y - 1) / 2;
                Normal
            }
            KeyCode::Char('g') => WaitG,
            _ => self.mode,
        };
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.mode {
            VimMode::Normal => self.handle_normal(key)?,
            VimMode::WaitG => {
                self.render_first()?;
                self.mode = VimMode::Normal
            }
            _ => {}
        };

        Ok(())
    }

    pub fn end(&mut self) -> io::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        self.end().unwrap();
    }
}
