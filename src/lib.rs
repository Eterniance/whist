pub mod game;
pub mod scoring;
pub mod contracts;
pub mod players;

use thiserror::Error;

use crate::contracts::hand::{HandBuildError, InputError};

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Too many players have been selected")]
    TooManyPlayer,
    #[error("This player name already exists")]
    PlayerAlreadyExists,
    #[error(transparent)]
    HandBuildError(#[from] HandBuildError),
    #[error(transparent)]
    InputError(#[from] InputError),
}
