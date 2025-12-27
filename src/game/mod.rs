pub mod hand;
pub mod players;
pub mod rules;

use thiserror::Error;

use crate::game::hand::InputError;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Expected 4 players")]
    TooManyPlayer,
    #[error("This player name already exists")]
    PlayerAlreadyExists,
    #[error("Something went wrong while building hand: {0}")]
    HandBuildError(String),
}

impl From<InputError> for GameError {
    fn from(_value: InputError) -> Self {
        Self::HandBuildError("Wrong user input".to_string())
    }
}
