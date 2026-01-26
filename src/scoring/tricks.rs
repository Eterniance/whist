use std::{convert::Infallible, fmt::Display};

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
pub struct Tricks(pub(crate) u8);

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

/// `Tricks` type derivation for score calculation taking the potential bid into account.
///
/// If a player make a bid of `Tricks(9)` in game with `Tricks(8)` required and achieve to collect
/// `Tricks(9)`, it is equivalent in scoring terms of collecting `Tricks(8)` in a game of `Tricks(8)`.
/// In both cases, the effective `Tricks` is 8.
///
/// The absolute field is the actual `Tricks` number collected. In the previous example, it would respectively
/// be `Tricks(9)` and `Tricks(8)`. This information is useful for calculating double points scenarios.
#[derive(Debug, Clone, Copy)]
pub struct CollectedTricks {
    /// Actual `Tricks` number collected by a player (or a Team) during a game.
    pub absolute: Tricks,
    /// Effective `Tricks` number regarding the potential bid and the minimum `Tricks` to win.
    pub effective: Tricks,
}

impl CollectedTricks {
    #[must_use]
    pub const fn new(absolute: Tricks, effective: Tricks) -> Self {
        Self {
            absolute,
            effective,
        }
    }

    /// Set both fields to the same value. Happens when no bid is made.
    #[must_use]
    pub const fn from_tricks(tricks: Tricks) -> Self {
        Self::new(tricks, tricks)
    }
}
