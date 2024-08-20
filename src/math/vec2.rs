use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use gl::RG;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec2 {
    x: f32,
    y: f32,
    
}



impl Vec2 {

    pub fn new(x: f32, y: f32) -> Self {
        Vec2 {
            x,
            y,
        }
    }

    pub fn zero() -> Self {
        Vec2 {
            x: 0.0,
            y: 0.0,
        }
    }

}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl AddAssign for Vec2 {

    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
        
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
        
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        };
            }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {

        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        };
        
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}