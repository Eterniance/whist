use crate::gamemodes::*;

pub enum GameRules {
    Dutch,
    French,
}

pub struct Game {
    gamemodes: Vec<Box<dyn Score>>
}

pub fn select_rules(rules: GameRules) -> Vec<Box<dyn Score>> {
    match rules {
        GameRules::Dutch => {
            let tricks_to_win = 8;
            let rules = Emballage::new(tricks_to_win, 2, 1);
            let hand = EmballageHand::new(tricks_to_win, rules);

            let seul = Seul::new(6, 6, 3, 8);

            vec![Box::new(hand), Box::new(seul)]
        }
        GameRules::French => {
            let tricks_to_win = 8;
            let rules = Emballage::new(tricks_to_win, 4, 2);
            let hand = EmballageHand::new(tricks_to_win, rules);

            let seul = Seul::new(5, 6, 3, 8);

            let picolo = Picolo::new(12);
            vec![Box::new(hand), Box::new(seul), Box::new(picolo)]
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
        let emballage_score = emballage.get_score(8);

        let expected_score = 2;

        assert_eq!(expected_score, emballage_score);
    }
}
