use super::*;

pub struct Seul {
    tricks_to_win: i16,
    min_points: i16,
    points_per_suppl_trick: i16,
    max_tricks_allowed: i16,
}

impl Score for Seul {
    fn calculate_score(&self, tricks: i16) -> (i16, GameResult) {
        let suppl_tricks = tricks - self.tricks_to_win;

        match suppl_tricks {
            0.. => {
                let points = self.min_points
                    + suppl_tricks.clamp(0, self.max_tricks_allowed) * self.points_per_suppl_trick;
                (points, GameResult::Win)
            }
            _ => {
                let points = self.min_points + suppl_tricks.abs() * self.points_per_suppl_trick;
                (points, GameResult::Lose)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const SEUL: Seul = Seul {
        tricks_to_win: 6,
        min_points: 6,
        points_per_suppl_trick: 3,
        max_tricks_allowed: 8,
    };

    #[test]
    fn test_seul_win() {
        let tricks = 8;
        let expected_score = 12;

        assert_eq!(expected_score, SEUL.get_score(tricks));
    }

        #[test]
    fn test_seul_lose() {
        let tricks = 3;
        let expected_score = -30;

        assert_eq!(expected_score, SEUL.get_score(tricks));
    }
}