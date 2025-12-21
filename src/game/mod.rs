pub mod rules;
pub mod game;


use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Expected 4 players")]
    TooManyPlayer,
    #[error("This player name already exists")]
    PlayerAlreadyExists,
}