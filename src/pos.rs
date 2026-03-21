use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    pub const fn new(x: u32, y: u32) -> Pos {
        Pos { x, y }
    }

    pub fn zero() -> Pos {
        Pos::new(0, 0)
    }

    pub fn up(mut self) -> Pos {
        self.y = self.y.saturating_sub(1);
        self
    }

    pub fn down(mut self) -> Pos {
        self.y += 1;
        self
    }

    pub fn left(mut self) -> Pos {
        self.x = self.x.saturating_sub(1);
        self
    }

    pub fn right(mut self) -> Pos {
        self.x += 1;
        self
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}
