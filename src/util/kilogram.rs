use std::ops::{Add, Sub};

#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct Kilogram(pub u32);

impl Sub for Kilogram {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Add for Kilogram {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
