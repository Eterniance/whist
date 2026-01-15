use std::{
    fmt::Debug,
    ops::{Deref, Div, Neg},
};
use crate::game::players::PlayerId;

pub(crate) mod emballage;
pub(crate) use emballage::Emballage;
pub(crate) mod picolo;
pub(crate) use picolo::Picolo;
pub(crate) mod seul;
pub(crate) use seul::Seul;
pub(crate) mod misere;
pub(crate) use misere::Misere;


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

pub trait Score: Debug {
    fn min_tricks(&self) -> i16;
    fn calculate_score(&self, tricks: i16) -> (i16, PointsCoefficient);

    fn get_single_player_score(&self, tricks: i16) -> i16 {
        let (points, coef) = self.calculate_score(tricks);
        points * i16::from(coef)
    }

    /// Computes the score for each of the four players, ensuring the total sum of scores is zero.
    ///
    /// This function takes a partial list of `(PlayerId, Tricks)` pairs, computes the score for
    /// each provided player using `get_single_player_score`, and assigns scores to the remaining
    /// players such that the overall score sum equals zero.
    ///
    /// # Behavior
    /// - Scores are calculated directly for players listed in `players_and_tricks`.
    /// - Players not listed receive an equal share of the negated sum of the provided players’ scores.
    /// - The function enforces that this remainder can be evenly divided among the missing players.
    /// - The final score array always has length 4 and sums to zero.
    ///
    /// # Errors
    /// Returns an error if the remaining score cannot be evenly distributed among the unspecified
    /// players (i.e., the total score is not divisible by the number of missing players).
    ///
    /// # Panics
    /// - Panics if a `PlayerId` index is out of bounds (expected to be `0..4`).
    /// - Panics if internal invariants are violated (final score sum is not zero).
    fn get_each_player_score(
        &self,
        players_and_tricks: &[(PlayerId, Tricks)],
    ) -> Result<[i16; 4], Box<dyn std::error::Error>> {
        let mut scores = [0; 4];
        let mut already_set_mask = 0;
        for (id, tricks) in players_and_tricks {
            let score = self.get_single_player_score(i16::from(*tricks));
            let idx = id.idx();
            scores[idx] = score;
            already_set_mask |= 1 << idx;
        }
        let others_score = scores.iter().sum::<i16>().neg();
        let div = i16::try_from(players_and_tricks.len()).expect("Length is between 0 and 4");

        if others_score % div != 0 {
            return Err("Score sum is non zero".into());
        };
        let others_score = others_score.div(div);

        (0..4)
            .filter(|&n| (already_set_mask & (1 << n)) == 0)
            .for_each(|i| scores[i] = others_score);

        assert_eq!(scores.iter().sum::<i16>(), 0);
        Ok(scores)
    }
}

macro_rules! score_enum {
    ($enum:ident {
        $ ( $variant:ident( $inner:ty ) ), + $(,)?
    }) => {
        impl Score for $enum {
            fn min_tricks(&self) -> i16 {
                match self {
                    $(
                        $enum::$variant(x) => x.min_tricks(),
                    )+
                }
            }

            fn calculate_score(&self, tricks: i16) -> (i16, PointsCoefficient) {
                match self {
                    $(
                        $enum::$variant(x) => x.calculate_score(tricks),
                    )+
               }
            }
        }
    };
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Gamemodes {
    Emballage(Emballage),
    Seul(Seul),
    Picolo(Picolo),
    Misere(Misere),
    GrandeMisere(Misere),
    GrandeMisereSurTrou(Misere),
}

impl Gamemodes {
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::Emballage(_) => "Emballage".to_string(),
            Self::Seul(_) => "Seul".to_string(),
            Self::Picolo(_) => "Picolo".to_string(),
            Self::Misere(_) => "Petite Misere".to_string(),
            Self::GrandeMisere(_) => "Grande Misere".to_string(),
            Self::GrandeMisereSurTrou(_) => "Grande Misere sur Trou".to_string(),
        }
    }
}

score_enum!(Gamemodes{
    Emballage(Emballage),
    Seul(Seul),
    Picolo(Picolo),
    Misere(Misere),
    GrandeMisere(Misere),
    GrandeMisereSurTrou(Misere),
});

#[cfg(test)]
mod tests {

}