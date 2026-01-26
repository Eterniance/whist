use crate::{CollectedTricks, scoring::{Score, Tricks}};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Emballage {
    tricks_to_win: Tricks,
    min_points: i16,
    points_per_suppl_trick: i16,
}

impl Emballage {
    #[must_use]
    pub const fn new(tricks_to_win: Tricks, min_points: i16, points_per_suppl_trick: i16) -> Self {
        Self {
            tricks_to_win,
            min_points,
            points_per_suppl_trick,
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Score for Emballage {
    fn calculate_score(&self, tricks: CollectedTricks) -> i16 {
        let capot = tricks.absolute == Tricks::MAX_TRICKS;

        let suppl_tricks = tricks.effective.as_i16() - self.tricks_to_win.as_i16();
        let points = self.min_points + suppl_tricks.abs() * self.points_per_suppl_trick;

        match suppl_tricks {
            0.. if capot => {
                2*(points - self.points_per_suppl_trick)
            }
            0.. => points,
            _ => -2*points,
        }
    }

    fn min_tricks(&self) -> Tricks {
        self.tricks_to_win
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMBALLAGE: Emballage = Emballage {
        tricks_to_win: Tricks(8),
        min_points: 2,
        points_per_suppl_trick: 1,
    };

    #[test]
    fn test_emballage_win() {
        let tricks = CollectedTricks::from_tricks(Tricks(8));
        let expected_score = 2;

        assert_eq!(expected_score, EMBALLAGE.calculate_score(tricks));
    }

    #[test]
    fn test_emballage_lost() {
        let tricks = CollectedTricks::from_tricks(Tricks(6));
        let expected_score = -8;

        assert_eq!(expected_score, EMBALLAGE.calculate_score(tricks));
    }

    #[test]
    fn test_emballage_capot() {
        let tricks = CollectedTricks::from_tricks(Tricks(13));
        let expected_score = 12;

        assert_eq!(expected_score, EMBALLAGE.calculate_score(tricks));
    }
}
