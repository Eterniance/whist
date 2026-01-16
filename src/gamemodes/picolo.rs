use super::{Debug, PointsCoefficient, Score};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Picolo {
    min_points: i16,
}

impl Picolo {
    #[must_use]
    pub const fn new(min_points: i16) -> Self {
        Self { min_points }
    }
}

impl Score for Picolo {
    fn calculate_score(&self, tricks: i16) -> (i16, PointsCoefficient) {
        if tricks == 1 {
            return (self.min_points, PointsCoefficient::One);
        }
        (self.min_points, PointsCoefficient::DoubleNeg)
    }

    fn min_tricks(&self) -> i16 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::gamemodes::score::Gamemodes;

    use super::*;

    const PICOLO: Picolo = Picolo { min_points: 12 };

    #[test]
    fn win() {
        let tricks = 1;
        let expected_score = 12;

        assert_eq!(expected_score, PICOLO.get_single_player_score(tricks));
        assert_eq!(
            expected_score,
            Gamemodes::Picolo(PICOLO).get_single_player_score(tricks)
        );
    }

    #[test]
    fn lose() {
        let tricks = 0;
        let expected_score = -24;

        assert_eq!(expected_score, PICOLO.get_single_player_score(tricks));

        let tricks = 3;
        assert_eq!(expected_score, PICOLO.get_single_player_score(tricks));
    }
}
