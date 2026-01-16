use crate::{
    contracts::{Contract, contractors::ContractorsScore, hand::InputError},
    scoring::{
        Gamemodes, TOTAL_TRICKS,
        gamemodes::{Emballage, Misere, Picolo, Seul},
    },
};
use strum_macros::{Display, EnumIter};

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
                contractors_kind: 2..=2,
            };
            let max_tricks_allowed = 8;
            let rules = Seul::new(6, 6, 3, max_tricks_allowed);

            let seul = Contract {
                max_bid: Some(max_tricks_allowed),
                gamemode: Gamemodes::Seul(rules),
                contractors_kind: 1..=1,
            };

            let rules = Misere::new(12);

            let petite_misere = Contract {
                max_bid: None,
                contractors_kind: 1..=3,
                gamemode: Gamemodes::Misere(rules),
            };

            let rules = Misere::new(24);

            let grande_misere = Contract {
                max_bid: None,
                contractors_kind: 1..=3,
                gamemode: Gamemodes::GrandeMisere(rules),
            };

            let rules = Misere::new(36);

            let grande_misere_sur_trou = Contract {
                max_bid: None,
                contractors_kind: 1..=3,
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
                contractors_kind: 2..=2,
            };
            let max_tricks_allowed = 8;
            let rules = Seul::new(6, 6, 3, max_tricks_allowed);

            let seul = Contract {
                max_bid: Some(max_tricks_allowed),
                gamemode: Gamemodes::Seul(rules),
                contractors_kind: 1..=1,
            };

            let rules = Picolo::new(12);

            let picolo = Contract {
                max_bid: None,
                gamemode: Gamemodes::Picolo(rules),
                contractors_kind: 1..=1,
            };

            vec![emballage, seul, picolo]
        }
    }
}

pub fn calculate_players_score(contractors: ContractorsScore) -> Result<[i16; 4], InputError> {
    let mut scores = [0; 4];
    match contractors {
        ContractorsScore::Solo(pias) => {
            let (idx, &score) = pias.as_components();
            if score % 3 != 0 {
                return Err(InputError::WrongScore);
            }
            for i in 0..4 {
                if i == *idx {
                    scores[i] = score;
                } else {
                    scores[i] = -score / 3;
                }
            }
            Ok(scores)
        }
        ContractorsScore::Team(pias1, pias2) => {
            let (idx_1, &score_1) = pias1.as_components();
            let (idx_2, &score_2) = pias2.as_components();
            if idx_1 == idx_2 {
                return Err(InputError::InvalidInput("Same player in team"));
            }
            if (score_1 + score_2) % 2 != 0 {
                return Err(InputError::WrongScore);
            }

            let other_players_score = -(score_1 + score_2) / 2;
            for i in 0..4 {
                if i == *idx_1 {
                    scores[i] = score_1;
                } else if i == *idx_2 {
                    scores[i] = score_2;
                } else {
                    scores[i] = other_players_score;
                }
            }
            Ok(scores)
        }
        ContractorsScore::Other(contractors) => match contractors.len() {
            1 => calculate_players_score(ContractorsScore::Solo(
                contractors.first().expect("Only one element").clone(),
            )),
            2 => calculate_players_score(ContractorsScore::Team(
                contractors.first().expect("Two elements").clone(),
                contractors.get(1).expect("Two elements").clone(),
            )),
            3 => {
                let mut last_player_idx = 6;
                let mut last_player_score = 0;
                for pias in contractors {
                    let (idx, &score) = pias.as_components();
                    last_player_idx -= idx;
                    last_player_score -= score;
                    scores[*idx] = score;
                }
                scores[last_player_idx] = last_player_score;
                Ok(scores)
            }
            _ => Err(InputError::WrongScore),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::Score;

    #[test]
    fn dutch() {
        let scorables = select_rules(&GameRules::Dutch);
        let emballage = &scorables[0];
        let emballage_score = emballage.gamemode.get_single_player_score(8);

        let expected_score = 2;

        assert_eq!(expected_score, emballage_score);
    }
}
