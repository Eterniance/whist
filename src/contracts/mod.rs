use crate::scoring::Score;
use std::ops::RangeInclusive;

pub mod contractors;
pub mod hand;

dyn_clone::clone_trait_object!(Score);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Contract {
    pub name: &'static str,
    pub max_bid: Option<i16>,
    pub contractors_kind: RangeInclusive<u8>,
    pub gamemode: Box<dyn Score>,
}

impl Contract {
    #[must_use]
    pub fn min_tricks(&self) -> i16 {
        self.gamemode.min_tricks()
    }
}
