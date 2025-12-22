use crate::gamemodes::*;

pub enum ContractorsKind {
    Solo,
    Team,
    Other,
}

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
            let hand_1 = Contract {
                max_bid: Some(TOTAL_TRICKS),
                gamemode: Box::new(rules),
                contractors_kind: ContractorsKind::Team,
            };
            let max_tricks_allowed = 8;
            let seul = Seul::new(6, 6, 3, max_tricks_allowed);

            let hand_2 = Contract {
                max_bid: Some(max_tricks_allowed),
                gamemode: Box::new(seul),
                contractors_kind: ContractorsKind::Solo,
            };

            vec![hand_1, hand_2]
        }
        GameRules::French => {
            todo!()
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
