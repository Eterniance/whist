use super::*;

#[derive(Debug)]
pub struct Emballage {
    tricks_to_win: i16,
    min_points: i16,
    points_per_suppl_trick: i16,
}

impl Emballage {
    pub fn new(tricks_to_win: i16, min_points: i16, points_per_suppl_trick: i16) -> Self {
        Self {
            tricks_to_win,
            min_points,
            points_per_suppl_trick,
        }
    }
}

#[derive(Debug)]
pub struct EmballageHand {
    pub bid: i16,
    rules: Emballage,
}

impl EmballageHand {
    pub fn new(bid: i16, rules: Emballage) -> Self {
        Self { bid, rules }
    }
}

impl Score for EmballageHand {
    fn calculate_score(&self, tricks: i16) -> (i16, GameResult) {
        let capot = tricks == TOTAL_TRICKS;

        let suppl_tricks = tricks - self.bid;
        let mut points =
            self.rules.min_points + suppl_tricks.abs() * self.rules.points_per_suppl_trick;

        let result = match suppl_tricks {
            0.. if capot => {
                points -= self.rules.points_per_suppl_trick;
                GameResult::Capot
            }
            0.. => GameResult::Win,
            _ => GameResult::Lose,
        };

        (points, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULES: Emballage = Emballage {
        tricks_to_win: 8,
        min_points: 2,
        points_per_suppl_trick: 1,
    };

    fn emballage(bid: i16) -> EmballageHand {
        EmballageHand { bid, rules: RULES }
    }

    #[test]
    fn test_emballage_win() {
        let tricks = 8;
        let expected_score = 2;

        assert_eq!(expected_score, emballage(8).get_score(tricks));
    }

    #[test]
    fn test_emballage_lost() {
        let tricks = 6;
        let expected_score = -8;

        assert_eq!(expected_score, emballage(8).get_score(tricks));
    }

    #[test]
    fn test_emballage_capot() {
        let tricks = 13;
        let expected_score = 12;

        assert_eq!(expected_score, emballage(8).get_score(tricks));
    }

    #[test]
    fn test_with_bid() {
        let tricks = 10;
        let expected_score = 3;

        assert_eq!(expected_score, emballage(9).get_score(tricks));
    }
}
