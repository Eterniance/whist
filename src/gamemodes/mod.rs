// #![allow(unused)]

use std::fmt::Debug;
pub(crate) mod emballage;
pub(crate) use emballage::Emballage;
pub(crate) mod picolo;
pub(crate) use picolo::Picolo;
pub(crate) mod seul;
pub(crate) use seul::Seul;
pub(crate) mod misere;
pub(crate) use misere::Misere;

pub const TOTAL_TRICKS: i16 = 13;

pub enum GameResult {
    Win,
    Lose,
    Capot,
}

pub trait Score: Debug {
    fn min_tricks(&self) -> i16;
    fn calculate_score(&self, tricks: i16) -> (i16, GameResult);

    fn get_score(&self, tricks: i16) -> i16 {
        let (points, result) = self.calculate_score(tricks);
        match result {
            GameResult::Win => points,
            GameResult::Lose => -2 * points,
            GameResult::Capot => 2 * points,
        }
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

            fn calculate_score(&self, tricks: i16) -> (i16, GameResult) {
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
