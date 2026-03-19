use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: u32,
    pub y: u32,
}

impl Vec2 {
    pub const fn new(x: u32, y: u32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2::new(0, 0)
    }

    pub fn up(mut self) -> Vec2 {
        self.y = self.y.saturating_sub(1);
        self
    }

    pub fn down(mut self) -> Vec2 {
        self.y += 1;
        self
    }

    pub fn left(mut self) -> Vec2 {
        self.x = self.x.saturating_sub(1);
        self
    }

    pub fn right(mut self) -> Vec2 {
        self.x += 1;
        self
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}
