use super::{Debug, PointsCoefficient, Score, TOTAL_TRICKS};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Emballage {
    tricks_to_win: i16,
    min_points: i16,
    points_per_suppl_trick: i16,
}

impl Emballage {
    #[must_use]
    pub const fn new(tricks_to_win: i16, min_points: i16, points_per_suppl_trick: i16) -> Self {
        Self {
            tricks_to_win,
            min_points,
            points_per_suppl_trick,
        }
    }
}

impl Score for Emballage {
    fn calculate_score(&self, tricks: i16) -> (i16, PointsCoefficient) {
        let capot = tricks == TOTAL_TRICKS;

        let suppl_tricks = tricks - self.tricks_to_win;
        let mut points = self.min_points + suppl_tricks.abs() * self.points_per_suppl_trick;

        let result = match suppl_tricks {
            0.. if capot => {
                points -= self.points_per_suppl_trick;
                PointsCoefficient::Double
            }
            0.. => PointsCoefficient::One,
            _ => PointsCoefficient::DoubleNeg,
        };

        (points, result)
    }

    fn min_tricks(&self) -> i16 {
        self.tricks_to_win
    }
}

#[cfg(test)]
mod tests {
    use crate::gamemodes::Gamemodes;

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

        assert_eq!(expected_score, EMBALLAGE.get_single_player_score(tricks));
        assert_eq!(
            expected_score,
            Gamemodes::Emballage(EMBALLAGE).get_single_player_score(tricks)
        );
    }

    #[test]
    fn test_emballage_lost() {
        let tricks = 6;
        let expected_score = -8;

        assert_eq!(expected_score, EMBALLAGE.get_single_player_score(tricks));
        assert_eq!(
            expected_score,
            Gamemodes::Emballage(EMBALLAGE).get_single_player_score(tricks)
        );
    }

    #[test]
    fn test_emballage_capot() {
        let tricks = 13;
        let expected_score = 12;

        assert_eq!(expected_score, EMBALLAGE.get_single_player_score(tricks));
        assert_eq!(
            expected_score,
            Gamemodes::Emballage(EMBALLAGE).get_single_player_score(tricks)
        );
    }
}
