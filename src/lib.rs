pub mod contracts;
pub mod players;
pub mod scoring;
pub use scoring::gamemodes;

use thiserror::Error;

use crate::contracts::hand::{HandBuildError, InputError};

const TOTAL_PLAYER: usize = 4;

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

#[cfg(test)]
mod test_utils {
    
    #[macro_export]
    macro_rules! p_and_t {
        ( $($trick:literal),+ $(,)? ) => {
            {let mut idx = 0;
                #[allow(unused)]
                [ $(
                    {
                        let pair = (PlayerId(idx), Tricks::try_from($trick).unwrap());
                        idx += 1;
                        pair
                    },
                )+]
            }
        };
    }
}
