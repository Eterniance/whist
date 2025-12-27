// #![allow(unused)]

use std::fmt::Debug;
pub(crate) mod emballage;
pub(crate) use emballage::Emballage;
pub(crate) mod picolo;
pub(crate) use picolo::Picolo;
pub(crate) mod seul;
pub(crate) use seul::Seul;

pub const TOTAL_TRICKS: i16 = 13;

pub enum GameResult {
    Win,
    Lose,
    Capot,
}

#[cfg(not(target_arch = "wasm32"))]
pub trait Score: Debug + Send + Sync {
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

#[cfg(target_arch = "wasm32")]
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
