pub mod contractors;
pub mod hand;
pub mod players;
pub mod rules;

use thiserror::Error;

use crate::game::hand::{HandBuildError, InputError};

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Too many players have been selected")]
    TooManyPlayer,
    #[error("This player name already exists")]
    PlayerAlreadyExists,
    #[error(transparent)]
    HandBuildError(#[from] HandBuildError),
    #[error(transparent)]
    InputError(#[from] InputError)
}
