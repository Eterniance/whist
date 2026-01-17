use crate::{
    players::PlayerId,
    scoring::{PointsCoefficient, Tricks},
};
use std::{fmt::Debug, ops::{Div, Neg}};
use dyn_clone::DynClone;

const TOTAL_PLAYER: usize = 4;

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait Score: Debug + DynClone {
    fn min_tricks(&self) -> i16;
    fn calculate_score(&self, tricks: i16) -> (i16, PointsCoefficient);

    fn get_single_player_score(&self, tricks: i16) -> i16 {
        let (points, coef) = self.calculate_score(tricks);
        points * i16::from(coef)
    }

    /// Computes the score for each of the four players, ensuring the total sum of scores is zero.
    ///
    /// This function takes a partial list of `(PlayerId, Tricks)` pairs, computes the score for
    /// each provided player using `get_single_player_score`, and assigns scores to the remaining
    /// players such that the overall score sum equals zero.
    ///
    /// # Behavior
    /// - Scores are calculated directly for players listed in `players_and_tricks`.
    /// - Players not listed receive an equal share of the negated sum of the provided players’ scores.
    /// - The function enforces that this remainder can be evenly divided among the missing players.
    /// - The final score array always has length 4 and sums to zero.
    ///
    /// # Errors
    /// Returns an error if the remaining score cannot be evenly distributed among the unspecified
    /// players (i.e., the total score is not divisible by the number of missing players).
    ///
    /// # Panics
    /// - Panics if a `PlayerId` index is out of bounds (expected to be `0..4`).
    /// - Panics if internal invariants are violated (final score sum is not zero).
    fn get_each_player_score(
        &self,
        players_and_tricks: &[(PlayerId, Tricks)],
    ) -> Result<[i16; 4], Box<dyn std::error::Error>> {
        let mut scores = [0; 4];
        let mut already_set_mask = 0;
        for (id, tricks) in players_and_tricks {
            let score = self.get_single_player_score(i16::from(*tricks));
            let idx = id.idx();
            scores[idx] = score;
            already_set_mask |= 1 << idx;
        }
        let others_score = scores.iter().sum::<i16>().neg();
        let div = 4 - i16::try_from(players_and_tricks.len()).expect("Length is between 0 and 4");

        if others_score % div != 0 {
            return Err("Score sum is non zero".into());
        }
        let others_score = others_score.div(div);

        (0..4)
            .filter(|&n| (already_set_mask & (1 << n)) == 0)
            .for_each(|i| scores[i] = others_score);

        // assert_eq!(scores.iter().sum::<i16>(), 0);
        Ok(scores)
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Scorable;

    macro_rules! p_and_t {
        ( $($trick:literal),+ $(,)? ) => {
            {let mut idx = 0;
                [ $(
                    {
                        let pair = (PlayerId(idx), Tricks::try_from($trick).unwrap());
                        idx += 1;
                        pair
                    },
                )+]
            }
        };
    }

    impl Score for Scorable {
        fn min_tricks(&self) -> i16 {
            0
        }

        fn calculate_score(&self, tricks: i16) -> (i16, PointsCoefficient) {
            let coef = match tricks {
                7..13 => PointsCoefficient::One,
                13 => PointsCoefficient::Double,
                0..=6 => PointsCoefficient::DoubleNeg,
                _ => unreachable!(),
            };
            (tricks, coef)
        }
    }

    #[test]
    fn null_sum() {
        let t = p_and_t![8, 8];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [8, 8, -8, -8]);
    }

    #[test]
    fn capot() {
        let t = p_and_t![13, 13];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [26, 26, -26, -26]);
    }

    #[test]
    fn fail_to_compute() {
        let t = p_and_t![8, 9];
        let scores = Scorable.get_each_player_score(&t);
        assert!(scores.is_err());
    }

    #[test]
    fn asymmetric_scores_1() {
        let t = p_and_t![9];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [9, -3, -3, -3]);
    }

    #[test]
    fn asymmetric_scores_2() {
        let t = p_and_t![6, 10];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [-12, 10, 1, 1]);
    }

    #[test]
    fn asymmetric_scores_3() {
        let t = p_and_t![8, 10, 12];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [8, 10, 12, -30]);
    }

    #[test]
    fn neg_to_pos() {
        let t = p_and_t![2, 4];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [-4, -8, 6, 6]);
    }

    #[test]
    fn disorder() {
        let t = [(PlayerId(2), Tricks::try_from(9).unwrap())];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [-3, -3, 9, -3]);
    }

    #[test]
    fn disorder_2() {
        let t = [
            (PlayerId(2), Tricks::try_from(6).unwrap()),
            (PlayerId(1), Tricks::try_from(8).unwrap()),
        ];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [2, 8, -12, 2]);
    }
}
