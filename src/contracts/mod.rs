use crate::scoring::{Gamemodes, Score};
use std::ops::RangeInclusive;

pub mod contractors;
pub mod hand;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Contract {
    pub max_bid: Option<i16>,
    pub contractors_kind: RangeInclusive<u8>,
    pub gamemode: Gamemodes,
}

impl Contract {
    #[must_use]
    pub fn min_tricks(&self) -> i16 {
        self.gamemode.min_tricks()
    }
}
