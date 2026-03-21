use overlay::Pager;
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    let buffer = std::fs::read_to_string("example.txt")?;
    let stdout = stdout();
    let mut pager = Pager::new(buffer, stdout);
    pager.start()
}
