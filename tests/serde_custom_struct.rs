mod utils;
#[cfg(test)]
#[cfg(feature = "serde")]
mod test {
    use crate::t;
    use whist_game::{CollectedTricks, Score, Tricks};

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct Chase4 {}

    #[typetag::serde]
    impl Score for Chase4 {
        fn min_tricks(&self) -> Tricks {
            Tricks::new(0).unwrap()
        }

        fn calculate_score(&self, tricks: CollectedTricks) -> i16 {
            match tricks.absolute.get() {
                4 => 21,
                3 => -15,
                2 => -10,
                1 => -5,
                0 => 0,
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn chase4_smoke() {
        let chase = Chase4 {};

        let tricks = CollectedTricks::from_tricks(t!(2));
        assert_eq!(chase.calculate_score(tricks), -10);
    }
}
