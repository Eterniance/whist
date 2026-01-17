use crate::scoring::{Score, Emballage, Misere, Seul, TOTAL_TRICKS};
use std::ops::RangeInclusive;

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

#[must_use]
pub fn default_contracts() -> Vec<Contract> {
    let tricks_to_win = 8;
    let rules = Emballage::new(tricks_to_win, 2, 1);
    let emballage = Contract {
        name: "Emballage",
        max_bid: Some(TOTAL_TRICKS),
        gamemode: Box::new(rules),
        contractors_kind: 2..=2,
    };
    let max_tricks_allowed = 8;
    let rules = Seul::new(6, 6, 3, max_tricks_allowed);

    let seul = Contract {
        name: "Seul",
        max_bid: Some(max_tricks_allowed),
        gamemode: Box::new(rules),
        contractors_kind: 1..=1,
    };

    let rules = Misere::new(12);

    let petite_misere = Contract {
        name: "Petite Misere",
        max_bid: None,
        contractors_kind: 1..=3,
        gamemode: Box::new(rules),
    };

    let rules = Misere::new(24);

    let grande_misere = Contract {
        name: "Grande Misere",
        max_bid: None,
        contractors_kind: 1..=3,
        gamemode: Box::new(rules),
    };

    let rules = Misere::new(36);

    let grande_misere_sur_trou = Contract {
        name: "Grande Misere sur trou",
        max_bid: None,
        contractors_kind: 1..=3,
        gamemode: Box::new(rules),
    };

    vec![
        emballage,
        seul,
        petite_misere,
        grande_misere,
        grande_misere_sur_trou,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dutch() {
        let scorables = default_contracts();
        let emballage = &scorables[0];
        let emballage_score = emballage.gamemode.get_single_player_score(8);

        let expected_score = 2;

        assert_eq!(expected_score, emballage_score);
    }
}
