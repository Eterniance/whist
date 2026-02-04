use crate::{
    CollectedTricks, TOTAL_PLAYERS,
    players::PlayerId,
    scoring::{CappedChase, ExactTricks, Score, Tricks, TricksChase},
};
use std::{ops::RangeInclusive};

pub mod hand;
pub use hand::{Hand, HandBuildError, HandRecap};

dyn_clone::clone_trait_object!(Score);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Contract {
    pub name: String,
    pub max_bid: Option<Tricks>,
    pub contractors_kind: RangeInclusive<u8>,
    pub gamemode: Box<dyn Score>,
}

impl Contract {
    #[must_use]
    fn min_tricks(&self) -> Tricks {
        self.gamemode.min_tricks()
    }

    /// Compute the score for each player. The array position corresponds to the player ID.
    ///
    /// # Errors
    /// Returns an error if the remaining score cannot be evenly distributed among the unspecified
    /// players (i.e., the total score is not divisible by the number of missing players).
    ///
    /// # Panics
    /// - Panics if a `PlayerId` index is out of bounds (expected to be `0..4`).
    /// - Panics if internal invariants are violated (final score sum is not zero).
    fn get_scores(
        &self,
        contractors_tricks: &[(PlayerId, Tricks)],
        bid: Option<Tricks>,
    ) -> Result<[i16; TOTAL_PLAYERS], Box<dyn std::error::Error>> {
        if contractors_tricks.is_empty() {
            return Err("Expected non zero length".into());
        }
        let players_and_tricks: Vec<_> = contractors_tricks
            .iter()
            .map(|&(id, tricks)| (id, self.get_collected_tricks(tricks, bid)))
            .collect();
        self.gamemode.get_each_player_score(&players_and_tricks)
    }

    /// The tricks number adjusted with the bid.
    ///
    /// Clamp the tricks number to the maximum allowed tricks if any
    /// and subtract the bid delta from the default minimum tricks.
    ///
    /// # Panic
    /// This function will panic if the inner value of `bid` is stricly smaller than
    /// `tricks`.
    #[must_use]
    fn get_collected_tricks(&self, tricks: Tricks, bid: Option<Tricks>) -> CollectedTricks {
        let clamped = match self.max_bid {
            None => return CollectedTricks::from_tricks(tricks),
            Some(max) => {
                let inner = tricks.get().clamp(0, max.get());
                Tricks::new(inner).expect("inner has been clamp between Tricks range")
            }
        };

        let effective = bid.map_or(clamped, |bid| {
            let diff = bid
                .checked_sub(self.min_tricks())
                .expect("Bid should be greater than min_tricks");
            clamped.saturating_sub(diff)
        });
        CollectedTricks::new(tricks, effective)
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn default_contracts() -> Vec<Contract> {
    let tricks_to_win = Tricks::new(8).expect("Withing range");
    let rules = TricksChase::new(tricks_to_win, 2, 1);
    let emballage = Contract {
        name: "Emballage".to_string(),
        max_bid: Some(Tricks::MAX_TRICKS),
        gamemode: Box::new(rules),
        contractors_kind: 2..=2,
    };
    let tricks_to_win = Tricks::new(6).expect("Withing range");
    let max_tricks_allowed = Tricks::new(8).expect("Within range");
    let rules = CappedChase::new(tricks_to_win, 6, 3, max_tricks_allowed);

    let seul = Contract {
        name: "Seul".to_string(),
        max_bid: Some(max_tricks_allowed),
        gamemode: Box::new(rules),
        contractors_kind: 1..=1,
    };

    let rules = ExactTricks::new(12, Tricks(0));

    let petite_misere = Contract {
        name: "Petite Misere".to_string(),
        max_bid: None,
        contractors_kind: 1..=3,
        gamemode: Box::new(rules),
    };

    let rules = ExactTricks::new(24, Tricks(0));

    let grande_misere = Contract {
        name: "Grande Misere".to_string(),
        max_bid: None,
        contractors_kind: 1..=3,
        gamemode: Box::new(rules),
    };

    let rules = ExactTricks::new(36, Tricks(0));

    let grande_misere_sur_trou = Contract {
        name: "Grande Misere sur trou".to_string(),
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
    use crate::{p_and_t, t};

    fn get_contract(name: &str) -> Contract {
        default_contracts()
            .into_iter()
            .find(|c| c.name == name)
            .unwrap()
    }

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

    #[test]
    fn adjusted_tricks_without_bid() {
        let contract = get_contract("Petite Misere");

        let tricks = t!(5);
        let computed_tricks = contract.get_collected_tricks(tricks, Some(t!(6)));
        assert_eq!(computed_tricks.absolute, computed_tricks.effective);
        assert_eq!(computed_tricks.absolute, tricks);
    }

    #[test]
    fn adjusted_tricks_with_bid() {
        let contract = get_contract("Emballage");

        let tricks = t!(10);
        let bid = t!(9);
        let computed_tricks = contract.get_collected_tricks(tricks, Some(bid));
        assert_eq!(computed_tricks.absolute, tricks);
        assert_eq!(computed_tricks.effective, bid);
    }

    #[test]
    fn adjusted_tricks_with_bid_saturating() {
        let contract = get_contract("Emballage");

        let tricks = t!(3);
        let bid = Some(Tricks::MAX_TRICKS);

        let computed_tricks = contract.get_collected_tricks(tricks, bid);

        assert_eq!(computed_tricks.absolute, tricks);
        assert_eq!(computed_tricks.effective, t!(0));
    }

    #[test]
    fn adjusted_tricks_clamp() {
        let contract = get_contract("Seul");

        let tricks = t!(12);
        let bid = t!(7);

        let computed_tricks = contract.get_collected_tricks(tricks, Some(bid));

        let min = contract.min_tricks();
        let diff = bid.checked_sub(min).unwrap();
        let expected = t!(8).saturating_sub(diff);

        assert_eq!(computed_tricks.absolute, tricks);
        assert_eq!(computed_tricks.effective, expected);
    }

    #[test]
    #[should_panic = "Bid should be greater than min_tricks"]
    fn bid_less_than_min_tricks() {
        let contract = get_contract("Emballage");
        let contractors_tricks = &p_and_t!(8, 8);
        let bid = Some(t!(7)); // Min tricks = 8
        let _ = contract.get_scores(contractors_tricks, bid).unwrap();
    }

    #[test]
    fn get_score_basic() {
        let contract = get_contract("Emballage");
        let contractors_tricks = &p_and_t!(8, 8);
        let bid = None;
        let scores = contract.get_scores(contractors_tricks, bid).unwrap();

        assert_eq!(scores, [2, 2, -2, -2]);
    }

    #[test]
    fn get_score_bid() {
        let contract = get_contract("Emballage");
        let contractors_tricks = &p_and_t!(9, 9);
        let bid = Some(t!(9));
        let scores = contract.get_scores(contractors_tricks, bid).unwrap();

        assert_eq!(scores, [2, 2, -2, -2]);
    }

    #[test]
    fn get_score_bid_lost() {
        let contract = get_contract("Emballage");
        let contractors_tricks = &p_and_t!(8, 8);
        let bid = Some(t!(9));
        let scores = contract.get_scores(contractors_tricks, bid).unwrap();

        assert_eq!(scores, [-6, -6, 6, 6]);
    }

    #[test]
    fn get_score_empty_contractors() {
        let contract = get_contract("Emballage");
        let contractors_tricks = &[];
        let bid = None;
        let scores = contract.get_scores(contractors_tricks, bid);

        assert!(scores.is_err());
    }

    #[test]
    fn get_score_capot() {
        let contract = get_contract("Emballage");
        let contractors_tricks = &p_and_t!(13, 13);
        let bid = Some(t!(8));
        let scores = contract.get_scores(contractors_tricks, bid).unwrap();

        assert_eq!(scores, [12, 12, -12, -12]);
    }

    #[test]
    fn get_score_capot_with_bid() {
        let contract = get_contract("Emballage");
        let contractors_tricks = &p_and_t!(13, 13);
        let bid = Some(t!(9));
        let scores = contract.get_scores(contractors_tricks, bid).unwrap();

        assert_eq!(scores, [10, 10, -10, -10]);
    }

    #[test]
    fn get_score_capot_with_bid_capped() {
        let contract = get_contract("Seul");
        let contractors_tricks = &p_and_t!(13);
        let bid = Some(t!(8));
        let scores = contract.get_scores(contractors_tricks, bid).unwrap();

        assert_eq!(scores, [6, -2, -2, -2]);
    }
}
