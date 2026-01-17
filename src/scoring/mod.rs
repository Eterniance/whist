use std::ops::Deref;

pub mod gamemodes;
pub(crate) use gamemodes::*;
mod score;
pub use score::Score;

pub const TOTAL_TRICKS: i16 = 13;

/// Number of tricks. This type represents a `u8`
/// that can only take values between `Self::MIN` and `Self::MAX`,
/// respectively representing the minimum and maximum possible tricks in a game of Whist.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Tricks(u8);

impl Tricks {
    const MIN: u8 = 0;
    const MAX: u8 = 13;
}

impl TryFrom<u8> for Tricks {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(format!("{value} is not in valid range").into());
        }
        Ok(Self(value))
    }
}

impl From<Tricks> for i16 {
    fn from(value: Tricks) -> Self {
        Self::from(value.0)
    }
}

impl Deref for Tricks {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(i8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PointsCoefficient {
    One = 1,
    Double = 2,
    DoubleNeg = -2,
}

impl PointsCoefficient {
    #[inline]
    #[must_use]
    pub const fn as_i8(self) -> i8 {
        self as i8
    }
}

impl From<PointsCoefficient> for i8 {
    #[inline]
    fn from(v: PointsCoefficient) -> Self {
        v as Self
    }
}

impl From<PointsCoefficient> for i16 {
    #[inline]
    fn from(v: PointsCoefficient) -> Self {
        Self::from(v as i8)
    }
}
impl From<PointsCoefficient> for i32 {
    #[inline]
    fn from(v: PointsCoefficient) -> Self {
        Self::from(v as i8)
    }
}
impl From<PointsCoefficient> for i64 {
    #[inline]
    fn from(v: PointsCoefficient) -> Self {
        Self::from(v as i8)
    }
}