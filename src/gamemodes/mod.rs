#![allow(unused)]

use std::fmt::Debug;
pub mod emballage;
pub use emballage::{Emballage};
pub mod picolo;
pub use picolo::Picolo;
pub mod seul;
pub use seul::Seul;

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
