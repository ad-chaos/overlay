use std::io::{self, Write, stdout};

use crossterm::event::{self, Event, KeyCode};
use crossterm::style::Print;
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown, ScrollUp,
    disable_raw_mode, enable_raw_mode, size,
};
use crossterm::{QueueableCommand, cursor::MoveTo, execute, queue};

#[derive(Default)]
struct Pager {
    buffer: String,
    columns: u16,
    rows: u16,
    col: u16,
    row: u16,
}

impl Pager {
    fn start(&mut self) -> io::Result<()> {
        let (columns, rows) = size()?;
        self.columns = columns;
        self.rows = rows;

        enable_raw_mode()?;
        let mut stdout = stdout();
        queue!(
            stdout,
            EnterAlternateScreen,
            Clear(ClearType::All),
            MoveTo(self.col, self.row),
        )?;

        for line in self.buffer.lines().skip(self.row as usize) {
            self.row += 1;
            if self.row > rows {
                break;
            }
            stdout
                .queue(Print(line))?
                .queue(MoveTo(self.col, self.row))?;
        }
        stdout.flush()?;
        Ok(())
    }

    fn run(&self) -> io::Result<()> {
        let mut stdout = stdout();
        loop {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') => {
                        stdout.queue(ScrollUp(1))?;
                    }
                    KeyCode::Char('k') => {
                        stdout.queue(ScrollDown(1))?;
                    }
                    _ => {}
                }
            }
            stdout.flush()?;
        }
    }

    fn end(&self) -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        self.end().unwrap();
    }
}

impl From<String> for Pager {
    fn from(buffer: String) -> Self {
        Self {
            buffer,
            ..Default::default()
        }
    }
}

fn main() -> io::Result<()> {
    let mut pager = Pager::from(std::fs::read_to_string("example.txt")?);
    pager.start()?;
    pager.run()?;
    Ok(())
}
