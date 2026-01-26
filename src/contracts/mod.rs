use crate::scoring::{CappedChase, ExactTricks, Score, Tricks, TricksChase};
use std::ops::RangeInclusive;

pub mod hand;

dyn_clone::clone_trait_object!(Score);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Contract {
    pub name: &'static str,
    pub max_bid: Option<Tricks>,
    pub contractors_kind: RangeInclusive<u8>,
    pub gamemode: Box<dyn Score>,
}

impl Contract {
    #[must_use]
    pub fn min_tricks(&self) -> Tricks {
        self.gamemode.min_tricks()
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn default_contracts() -> Vec<Contract> {
    let tricks_to_win = Tricks::new(8).expect("Withing range");
    let rules = TricksChase::new(tricks_to_win, 2, 1);
    let emballage = Contract {
        name: "Emballage",
        max_bid: Some(Tricks::MAX_TRICKS),
        gamemode: Box::new(rules),
        contractors_kind: 2..=2,
    };
    let tricks_to_win = Tricks::new(6).expect("Withing range");
    let max_tricks_allowed = Tricks::new(8).expect("Within range");
    let rules = CappedChase::new(tricks_to_win, 6, 3, max_tricks_allowed);

    let seul = Contract {
        name: "Seul",
        max_bid: Some(max_tricks_allowed),
        gamemode: Box::new(rules),
        contractors_kind: 1..=1,
    };

    let rules = ExactTricks::new(12);

    let petite_misere = Contract {
        name: "Petite Misere",
        max_bid: None,
        contractors_kind: 1..=3,
        gamemode: Box::new(rules),
    };

    let rules = ExactTricks::new(24);

    let grande_misere = Contract {
        name: "Grande Misere",
        max_bid: None,
        contractors_kind: 1..=3,
        gamemode: Box::new(rules),
    };

    let rules = ExactTricks::new(36);

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
    use crate::t;

    #[test]
    fn dutch() {
        let scorables = default_contracts();
        let emballage = &scorables[0];
        let emballage_score = emballage
            .gamemode
            .calculate_score(crate::CollectedTricks::from_tricks(t!(8)));

        let expected_score = 2;

        assert_eq!(expected_score, emballage_score);
    }
}
