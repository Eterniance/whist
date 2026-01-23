use crate::{
    Tricks,
    scoring::{PointsCoefficient, Score},
};

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

#[cfg_attr(feature = "serde", typetag::serde)]
impl Score for Picolo {
    fn calculate_score(&self, tricks: Tricks) -> (i16, PointsCoefficient) {
        if tricks.get() == 1 {
            return (self.min_points, PointsCoefficient::One);
        }
        (self.min_points, PointsCoefficient::DoubleNeg)
    }

    fn min_tricks(&self) -> Tricks {
        Tricks::new(1).expect("Withing range")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PICOLO: Picolo = Picolo { min_points: 12 };

    #[test]
    fn win() {
        let tricks = Tricks::new(1).expect("Within range");
        let expected_score = 12;

        assert_eq!(expected_score, PICOLO.get_single_player_score(tricks));
    }

    #[test]
    fn lose() {
        let tricks = Tricks::new(0).expect("Within range");
        let expected_score = -24;

        assert_eq!(expected_score, PICOLO.get_single_player_score(tricks));

        let tricks = Tricks::new(3).expect("Within range");
        assert_eq!(expected_score, PICOLO.get_single_player_score(tricks));
    }
}
