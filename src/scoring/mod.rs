use std::{convert::Infallible, fmt::Display};
pub mod gamemodes;
pub(crate) use gamemodes::*;
mod score;
pub use score::Score;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TricksOutOfRange(pub u8);

impl Display for TricksOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Expected value in ({}..={}), got {}",
            Tricks::MIN,
            Tricks::MAX,
            self.0
        )
    }
}

impl std::error::Error for TricksOutOfRange {}

impl From<Infallible> for TricksOutOfRange {
    fn from(_value: Infallible) -> Self {
        unreachable!()
    }
}

/// Tricks number. This type represents a `u8`
/// that can only take values between `Self::MIN` and `Self::MAX`,
/// respectively representing the minimum and maximum possible tricks in a game of Whist.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tricks(u8);

impl Tricks {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 13;

    pub const MIN_TRICKS: Self = Self(Self::MIN);
    pub const MAX_TRICKS: Self = Self(Self::MAX);

    /// Creates a new `Tricks` value.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is outside the valid range
    /// `[Self::MIN, Self::MAX]`.
    pub fn new(value: u8) -> Result<Self, TricksOutOfRange> {
        value.try_into()
    }

    /// Returns the underlying number of tricks as a `u8`.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }

    #[must_use]
    /// Returns the number of tricks as an `i16`.
    ///
    /// This is a lossless conversion.
    pub const fn as_i16(self) -> i16 {
        self.0 as i16
    }

    /// Adds another value to this number of tricks, with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the operand cannot be converted to `Tricks`, or if
    /// the resulting value is outside the valid range.
    pub fn checked_add<T>(self, other: T) -> Result<Self, TricksOutOfRange>
    where
        T: TryInto<Self>,
        TricksOutOfRange: From<<T as TryInto<Self>>::Error>,
    {
        let value = self.0 + other.try_into()?.0;
        Self::try_from(value)
    }

    /// Adds two `Tricks` values, clamping the result to the valid range.
    ///
    /// The result is capped at `Self::MAX`.
    #[must_use]
    pub fn saturating_add(self, other: Self) -> Self {
        let value = (self.0 + other.0).clamp(Self::MIN, Self::MAX);
        Self(value)
    }

    /// Subtracts another value from this number of tricks, with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the operand cannot be converted to `Tricks`, or if
    /// the subtraction would underflow or produce an invalid value.
    pub fn checked_sub<T>(self, other: T) -> Result<Self, TricksOutOfRange>
    where
        T: TryInto<Self>,
        TricksOutOfRange: From<<T as TryInto<Self>>::Error>,
    {
        let other = other.try_into()?;
        let value = self
            .0
            .checked_sub(other.0)
            .ok_or_else(|| TricksOutOfRange(self.0.wrapping_sub(other.0)))?;
        Self::try_from(value)
    }

    /// Subtracts two `Tricks` values, saturating at zero.
    #[must_use]
    pub fn saturating_sub(self, other: Self) -> Self {
        let value = self.0.saturating_sub(other.0).min(Self::MAX);
        Self(value)
    }
}

impl TryFrom<u8> for Tricks {
    type Error = TricksOutOfRange;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(TricksOutOfRange(value));
        }
        Ok(Self(value))
    }
}

impl From<Tricks> for u8 {
    fn from(t: Tricks) -> Self {
        t.0
    }
}

impl From<Tricks> for i16 {
    fn from(value: Tricks) -> Self {
        Self::from(value.0)
    }
}

impl Display for Tricks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

macro_rules! impl_from {
    ($self:ty: $($target:ty)+) => {
        $(
            impl From<$self> for $target {
                #[inline]
                fn from(v: $self) -> Self {
                    v as $target
                }
            }
        )*
    };
}
impl_from!(PointsCoefficient: i16 i32 i64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::t;

    #[test]
    fn trivial_cases() {
        let a = t!(3);
        let b = t!(4);

        let sum = a.checked_add(b).unwrap();
        assert_eq!(sum, t!(7));
        assert_eq!(sum.get(), 7);

        let sat_sum = a.saturating_add(b);
        assert_eq!(sum, sat_sum);

        let diff = b.checked_sub(a).unwrap();
        assert_eq!(diff, t!(1));

        let sat_diff = b.saturating_sub(a);
        assert_eq!(diff, sat_diff);
    }

    #[test]
    fn checked_add_with_u8_rhs_out_of_range_err() {
        let a = t!(5);

        let err = a.checked_add(14u8).unwrap_err();
        assert_eq!(err.0, 14);
    }

    #[test]
    fn checked_add_sum_out_of_range_err() {
        let a = t!(13);

        let err = a.checked_add(t!(13)).unwrap_err();
        assert_eq!(err.0, 26);
    }

    #[test]
    fn saturating_add_saturates() {
        let a = t!(13);
        let b = t!(11);

        let sum = a.saturating_add(b);
        assert_eq!(sum.get(), 13);
    }

    #[test]
    fn checked_sub_with_u8_rhs_out_of_range_err() {
        let a = t!(10);

        let err = a.checked_sub(14u8).unwrap_err();
        assert_eq!(err.0, 14);
    }

    #[test]
    fn checked_sub_underflow() {
        let a = t!(0);

        let err = a.checked_sub(t!(1)).unwrap_err();
        assert_eq!(err.0, 255);
    }

    #[test]
    fn saturating_sub_ok() {
        let a = t!(10);
        let b = t!(3);

        let diff = a.saturating_sub(b);
        assert_eq!(diff.get(), 7);
    }

    #[test]
    fn saturating_sub_saturates_at_zero() {
        let a = t!(2);
        let b = t!(13);

        let diff = a.saturating_sub(b);
        assert_eq!(diff.get(), 0);
        assert_eq!(diff, Tricks::MIN_TRICKS);
    }
}
