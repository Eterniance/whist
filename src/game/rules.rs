use crate::{
    game::contractors::ContractorsKind,
    gamemodes::{Emballage, Gamemodes, Misere, Picolo, Score, Seul, TOTAL_TRICKS},
};

use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Contract {
    pub max_bid: Option<i16>,
    pub contractors_kind: ContractorsKind,
    pub gamemode: Gamemodes,
}

impl Contract {
    #[must_use]
    pub fn min_tricks(&self) -> i16 {
        self.gamemode.min_tricks()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GameRules {
    Dutch,
    French,
}

#[must_use]
pub fn select_rules(rules: &GameRules) -> Vec<Contract> {
    match rules {
        GameRules::Dutch => {
            let tricks_to_win = 8;
            let rules = Emballage::new(tricks_to_win, 2, 1);
            let emballage = Contract {
                max_bid: Some(TOTAL_TRICKS),
                gamemode: Gamemodes::Emballage(rules),
                contractors_kind: ContractorsKind::Team,
            };
            let max_tricks_allowed = 8;
            let rules = Seul::new(6, 6, 3, max_tricks_allowed);

            let seul = Contract {
                max_bid: Some(max_tricks_allowed),
                gamemode: Gamemodes::Seul(rules),
                contractors_kind: ContractorsKind::Solo,
            };

            let rules = Misere::new(12);

            let petite_misere = Contract {
                max_bid: None,
                contractors_kind: ContractorsKind::Other,
                gamemode: Gamemodes::Misere(rules),
            };

            let rules = Misere::new(24);

            let grande_misere = Contract {
                max_bid: None,
                contractors_kind: ContractorsKind::Other,
                gamemode: Gamemodes::GrandeMisere(rules),
            };

            let rules = Misere::new(36);

            let grande_misere_sur_trou = Contract {
                max_bid: None,
                contractors_kind: ContractorsKind::Other,
                gamemode: Gamemodes::GrandeMisereSurTrou(rules),
            };

            vec![
                emballage,
                seul,
                petite_misere,
                grande_misere,
                grande_misere_sur_trou,
            ]
        }
        GameRules::French => {
            let tricks_to_win = 8;
            let rules = Emballage::new(tricks_to_win, 2, 1);
            let emballage = Contract {
                max_bid: Some(TOTAL_TRICKS),
                gamemode: Gamemodes::Emballage(rules),
                contractors_kind: ContractorsKind::Team,
            };
            let max_tricks_allowed = 8;
            let rules = Seul::new(6, 6, 3, max_tricks_allowed);

            let seul = Contract {
                max_bid: Some(max_tricks_allowed),
                gamemode: Gamemodes::Seul(rules),
                contractors_kind: ContractorsKind::Solo,
            };

            let rules = Picolo::new(12);

            let picolo = Contract {
                max_bid: None,
                gamemode: Gamemodes::Picolo(rules),
                contractors_kind: ContractorsKind::Solo,
            };

            vec![emballage, seul, picolo]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dutch() {
        let scorables = select_rules(&GameRules::Dutch);
        let emballage = &scorables[0];
        let emballage_score = emballage.gamemode.get_score(8);

        let expected_score = 2;

        assert_eq!(expected_score, emballage_score);
    }
}
