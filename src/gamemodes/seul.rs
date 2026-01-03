use super::{Debug, GameResult, Score};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Seul {
    tricks_to_win: i16,
    min_points: i16,
    points_per_suppl_trick: i16,
    max_tricks_allowed: i16,
}

impl Seul {
    #[must_use]
    pub const fn new(
        tricks_to_win: i16,
        min_points: i16,
        points_per_suppl_trick: i16,
        max_tricks_allowed: i16,
    ) -> Self {
        Self {
            tricks_to_win,
            min_points,
            points_per_suppl_trick,
            max_tricks_allowed,
        }
    }
}

impl Score for Seul {
    fn calculate_score(&self, tricks: i16) -> (i16, GameResult) {
        let suppl_tricks = tricks.clamp(0, self.max_tricks_allowed) - self.tricks_to_win;

        if let 0.. = suppl_tricks {
            let points = self.min_points + suppl_tricks * self.points_per_suppl_trick;
            (points, GameResult::Win)
        } else {
            let points = self.min_points + suppl_tricks.abs() * self.points_per_suppl_trick;
            (points, GameResult::Lose)
        }
    }

    fn min_tricks(&self) -> i16 {
        self.tricks_to_win
    }

    fn name(&self) -> String {
        "Seul".to_string()
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

    #[test]
    fn test_seul_win_too_much() {
        let tricks = 9;
        let expected_score = 12;

        assert_eq!(expected_score, SEUL.get_score(tricks));
    }
}
