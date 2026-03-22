use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Pos {
    pub col: usize,
    pub line: usize,
}

impl Pos {
    pub const fn new(x: usize, y: usize) -> Pos {
        Pos { col: x, line: y }
    }

    pub fn zero() -> Pos {
        Pos::new(0, 0)
    }

    pub fn up(mut self) -> Pos {
        self.line = self.line.saturating_sub(1);
        self
    }

    pub fn down(mut self) -> Pos {
        self.line += 1;
        self
    }

    pub fn left(mut self) -> Pos {
        self.col = self.col.saturating_sub(1);
        self
    }

    pub fn right(mut self) -> Pos {
        self.col += 1;
        self
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            col: self.col + rhs.col,
            line: self.line + rhs.line,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            col: self.col - rhs.col,
            line: self.line - rhs.line,
        }
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}
