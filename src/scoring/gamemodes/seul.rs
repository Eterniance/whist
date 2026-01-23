use crate::scoring::{PointsCoefficient, Score, Tricks};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Seul {
    tricks_to_win: Tricks,
    min_points: i16,
    points_per_suppl_trick: i16,
    max_tricks_allowed: Tricks,
}

impl Seul {
    #[must_use]
    pub const fn new(
        tricks_to_win: Tricks,
        min_points: i16,
        points_per_suppl_trick: i16,
        max_tricks_allowed: Tricks,
    ) -> Self {
        Self {
            tricks_to_win,
            min_points,
            points_per_suppl_trick,
            max_tricks_allowed,
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Score for Seul {
    fn calculate_score(&self, tricks: Tricks) -> (i16, PointsCoefficient) {
        let suppl_tricks = tricks.as_i16().clamp(0, self.max_tricks_allowed.as_i16())
            - self.tricks_to_win.as_i16();

        if let 0.. = suppl_tricks {
            let points = self.min_points + suppl_tricks * self.points_per_suppl_trick;
            (points, PointsCoefficient::One)
        } else {
            let points = self.min_points + suppl_tricks.abs() * self.points_per_suppl_trick;
            (points, PointsCoefficient::DoubleNeg)
        }
    }

    fn min_tricks(&self) -> Tricks {
        self.tricks_to_win
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEUL: Seul = Seul {
        tricks_to_win: Tricks(6),
        min_points: 6,
        points_per_suppl_trick: 3,
        max_tricks_allowed: Tricks(8),
    };

    #[test]
    fn test_seul_win() {
        let tricks = Tricks(8);
        let expected_score = 12;

        assert_eq!(expected_score, SEUL.get_single_player_score(tricks));
    }

    #[test]
    fn test_seul_lose() {
        let tricks = Tricks(3);
        let expected_score = -30;

        assert_eq!(expected_score, SEUL.get_single_player_score(tricks));
    }

    #[test]
    fn test_seul_win_too_much() {
        let tricks = Tricks(9);
        let expected_score = 12;

        assert_eq!(expected_score, SEUL.get_single_player_score(tricks));
    }
}
