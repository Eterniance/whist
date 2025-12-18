use super::*;

pub struct Emballage {
    tricks_to_win: i16,
    min_points: i16,
    points_per_suppl_trick: i16,
}

impl Score for Emballage {
    fn calculate_score(&self, tricks: i16) -> (i16, GameResult) {
        let capot = tricks == TOTAL_TRICKS;

        let suppl_tricks = tricks - self.tricks_to_win;
        let mut points = self.min_points + suppl_tricks.abs() * self.points_per_suppl_trick;

        let result = match suppl_tricks {
            0.. if capot => {
                points -= self.points_per_suppl_trick;
                GameResult::Capot
            }
            0.. => GameResult::Win,
            _ => GameResult::Lose,
        };

        (points, result)
    }

    fn get_score(&self, tricks: i16) -> i16 {
        let (points, result) = self.calculate_score(tricks);
        match result {
            super::GameResult::Win => points,
            super::GameResult::Lose => -2 * points,
            super::GameResult::Capot => 2 * points,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMBALLAGE: Emballage = Emballage {
        tricks_to_win: 8,
        min_points: 2,
        points_per_suppl_trick: 1,
    };

    #[test]
    fn test_emballage_win() {
        let tricks = 8;
        let expected_score = 2;

        assert_eq!(expected_score, EMBALLAGE.get_score(tricks));
    }

    #[test]
    fn test_emballage_lost() {
        let tricks = 6;
        let expected_score = -8;

        assert_eq!(expected_score, EMBALLAGE.get_score(tricks));
    }

    #[test]
    fn test_emballage_capot() {
        let tricks = 13;
        let expected_score = 12;

        assert_eq!(expected_score, EMBALLAGE.get_score(tricks));
    }
}
