use crate::{game::players::Contractors, gamemodes::*};

#[derive(Debug)]
pub enum ContractorsKind {
    Solo,
    Team,
    Other,
}

impl PartialEq<Contractors> for ContractorsKind {
    fn eq(&self, other: &Contractors) -> bool {
        matches!(
            (self, other),
            (ContractorsKind::Solo, Contractors::Solo(_))
                | (ContractorsKind::Team, Contractors::Team(_, _))
                | (ContractorsKind::Other, Contractors::Other)
        )
    }
}

#[derive(Debug)]
pub struct Contract {
    pub max_bid: Option<i16>,
    pub contractors_kind: ContractorsKind,
    gamemode: Box<dyn Score>,
}

impl Contract {
    pub fn get_score(&self, tricks: i16) -> i16 {
        self.gamemode.get_score(tricks)
    }

    pub fn min_tricks(&self) -> i16 {
        self.gamemode.min_tricks()
    }
}

pub enum GameRules {
    Dutch,
    French,
}

pub fn select_rules(rules: GameRules) -> Vec<Contract> {
    match rules {
        GameRules::Dutch => {
            let tricks_to_win = 8;
            let rules = Emballage::new(tricks_to_win, 2, 1);
            let emballage = Contract {
                max_bid: Some(TOTAL_TRICKS),
                gamemode: Box::new(rules),
                contractors_kind: ContractorsKind::Team,
            };
            let max_tricks_allowed = 8;
            let rules = Seul::new(6, 6, 3, max_tricks_allowed);

            let seul = Contract {
                max_bid: Some(max_tricks_allowed),
                gamemode: Box::new(rules),
                contractors_kind: ContractorsKind::Solo,
            };

            vec![emballage, seul]
        }
        GameRules::French => {
            let tricks_to_win = 8;
            let rules = Emballage::new(tricks_to_win, 2, 1);
            let emballage = Contract {
                max_bid: Some(TOTAL_TRICKS),
                gamemode: Box::new(rules),
                contractors_kind: ContractorsKind::Team,
            };
            let max_tricks_allowed = 8;
            let rules = Seul::new(6, 6, 3, max_tricks_allowed);

            let seul = Contract {
                max_bid: Some(max_tricks_allowed),
                gamemode: Box::new(rules),
                contractors_kind: ContractorsKind::Solo,
            };

            let rules = Picolo::new(12);

            let picolo = Contract {
                max_bid: None,
                gamemode: Box::new(rules),
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
        let scorables = select_rules(GameRules::Dutch);
        let emballage = &scorables[0];
        let emballage_score = emballage.gamemode.get_score(8);

        let expected_score = 2;

        assert_eq!(expected_score, emballage_score);
    }
}
