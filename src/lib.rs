pub mod contracts;
pub mod players;
mod scoring;

pub use self::{contracts::*, scoring::*, players::*};
pub use crate::contracts::hand::{HandBuildError, InputError};

use thiserror::Error;

const TOTAL_PLAYERS: usize = 4;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Expected {TOTAL_PLAYERS} players, got {0}")]
    PlayersNotSet(usize),
    #[error("This player name already exists")]
    PlayerAlreadyExists,
    #[error(transparent)]
    HandBuildError(#[from] HandBuildError),
    #[error(transparent)]
    InputError(#[from] InputError),
}

#[cfg(test)]
pub mod test_utils {

    #[macro_export]
    macro_rules! p {
        ( $($idx:literal),+ $(,)? ) => {
            [$($crate::players::PlayerId($idx)),+]
        };
    }

    #[macro_export]
    macro_rules! t {
        ($v:literal) => {
            $crate::Tricks::new($v).unwrap()
        };
    }

    #[macro_export]
    macro_rules! p_and_t {
        (collected $($trick:literal),+ $(,)? ) => {
            {let mut idx = 0;
                #[allow(unused)]
                [ $(
                    {
                        let collected = $crate::scoring::CollectedTricks::from_tricks($crate::t!($trick));
                        let pair = ($crate::players::PlayerId(idx), collected);
                        idx += 1;
                        pair
                    },
                )+]
            }
        };
        ( $($trick:literal),+ $(,)? ) => {
            {let mut idx = 0;
                #[allow(unused)]
                [ $(
                    {
                        let pair = ($crate::players::PlayerId(idx), $crate::t!($trick));
                        idx += 1;
                        pair
                    },
                )+]
            }
        };
    }
}
