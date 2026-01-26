use crate::{CollectedTricks, scoring::{Score, Tricks}};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Misere {
    min_points: i16,
}

impl Misere {
    #[must_use]
    pub const fn new(min_points: i16) -> Self {
        Self { min_points }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Score for Misere {
    fn calculate_score(&self, tricks: CollectedTricks) -> i16 {
        if tricks.absolute.get() == 0 {
            return self.min_points;
        }
        -2*self.min_points
    }

    fn min_tricks(&self) -> Tricks {
        Tricks::new(0).expect("Within range")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MISERE: Misere = Misere { min_points: 12 };

    #[test]
    fn win() {
        let tricks = CollectedTricks::from_tricks(Tricks(0));
        let expected_score = 12;

        assert_eq!(expected_score, MISERE.calculate_score(tricks));
    }

    #[test]
    fn lose() {
        let tricks = CollectedTricks::from_tricks(Tricks(1));
        let expected_score = -24;

        assert_eq!(expected_score, MISERE.calculate_score(tricks));

        let tricks = CollectedTricks::from_tricks(Tricks(3));
        assert_eq!(expected_score, MISERE.calculate_score(tricks));
    }
}
