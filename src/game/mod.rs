pub mod rules;
pub mod players;
pub mod hand;


use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Expected 4 players")]
    TooManyPlayer,
    #[error("This player name already exists")]
    PlayerAlreadyExists,
    #[error("Something went wrong while building hand: {0}")]
    HandBuildError(String),
}