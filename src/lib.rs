pub mod contracts;
pub mod players;
pub mod scoring;
pub use scoring::{Tricks, gamemodes};

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
pub(crate) mod test_utils {

    #[macro_export]
    macro_rules! p {
    ( $($idx:literal),+ $(,)? ) => {
        [$(PlayerId::new($idx)),+]
    };
}

    #[macro_export]
    macro_rules! t {
        ($v:literal) => {
            Tricks::new($v).unwrap()
        };
    }

    #[macro_export]
    macro_rules! p_and_t {
        ( $($trick:literal),+ $(,)? ) => {
            {let mut idx = 0;
                #[allow(unused)]
                [ $(
                    {
                        let pair = (PlayerId(idx), $crate::t!($trick));
                        idx += 1;
                        pair
                    },
                )+]
            }
        };
    }
}
