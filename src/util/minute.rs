use std::ops::{Add, Sub};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Minute(pub u32);

impl Add for Minute {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Minute {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
