use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: u128,
    pub y: u128,
}

impl Vector {
    pub fn new() -> Self {
        Vector { x: 0, y: 0 }
    }

    pub fn from(x: u128, y: u128) -> Self {
        Vector { x, y }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<u128> for Vector {
    fn mul(self, rhs: u128) -> Vector {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }

    type Output = Self;
}

impl Div<u128> for Vector {
    fn div(self, rhs: u128) -> Vector {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }

    type Output = Self;
}
