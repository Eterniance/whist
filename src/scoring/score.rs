use crate::{
    TOTAL_PLAYERS,
    players::PlayerId,
    scoring::{Tricks, tricks::CollectedTricks},
};
use dyn_clone::DynClone;
use std::{
    fmt::Debug,
    ops::{Div, Neg},
};

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait Score: Debug + DynClone {
    /// The minimum tricks to win.
    fn min_tricks(&self) -> Tricks;
    /// Gives the score based on tricks number.
    fn calculate_score(&self, tricks: CollectedTricks) -> i16;

    #[allow(clippy::doc_lazy_continuation)]
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
    /// - Returns an error if a `PlayerId` is duplicated.
    /// - Returns an error if the remaining score cannot be evenly distributed among the unspecified
    /// players (i.e., the total score is not divisible by the number of missing players).
    ///
    /// # Panics
    /// - Panics if a `PlayerId` index is out of bounds (expected to be `0..4`).
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn get_each_player_score(
        &self,
        players_and_tricks: &[(PlayerId, CollectedTricks)],
    ) -> Result<[i16; TOTAL_PLAYERS], Box<dyn std::error::Error>> {
        let mut scores = [0; TOTAL_PLAYERS];
        let mut already_set_mask = 0;
        for (id, tricks) in players_and_tricks {
            let idx = id.idx();
            let already_set_bit = 1 << idx;
            if (already_set_mask & already_set_bit) != 0 {
                return Err("Duplicate PlayerId".into());
            }
            already_set_mask |= already_set_bit;

            scores[idx] = self.calculate_score(*tricks);
        }
        let others_score = scores.iter().sum::<i16>().neg();
        let div = TOTAL_PLAYERS as i16
            - i16::try_from(players_and_tricks.len()).expect("Length should be less than i16::MAX");

        if others_score % div != 0 {
            return Err("Score sum is non zero".into());
        }
        let others_score = others_score.div(div);

        (0..TOTAL_PLAYERS)
            .filter(|&n| (already_set_mask & (1 << n)) == 0)
            .for_each(|i| scores[i] = others_score);

        Ok(scores)
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::{Tricks, p_and_t, t};

    use super::*;

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    struct Scorable;

    #[cfg_attr(feature = "serde", typetag::serde)]
    impl Score for Scorable {
        fn min_tricks(&self) -> Tricks {
            Tricks(0)
        }

        fn calculate_score(&self, tricks: CollectedTricks) -> i16 {
            let tricks = tricks.absolute;
            match tricks.get() {
                7..13 => tricks.as_i16(),
                13 => 2 * tricks.as_i16(),
                0..=6 => -2 * tricks.as_i16(),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn null_sum() {
        let t = p_and_t![collected 8, 8];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [8, 8, -8, -8]);
    }

    #[test]
    fn capot() {
        let t = p_and_t![collected 13, 13];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [26, 26, -26, -26]);
    }

    #[test]
    fn fail_to_compute() {
        let t = p_and_t![collected 8, 9];
        let scores = Scorable.get_each_player_score(&t);
        assert!(scores.is_err());
    }

    #[test]
    fn asymmetric_scores_1() {
        let t = p_and_t![collected 9];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [9, -3, -3, -3]);
    }

    #[test]
    fn asymmetric_scores_2() {
        let t = p_and_t![collected 6, 10];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [-12, 10, 1, 1]);
    }

    #[test]
    fn asymmetric_scores_3() {
        let t = p_and_t![collected 8, 10, 12];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [8, 10, 12, -30]);
    }

    #[test]
    fn neg_to_pos() {
        let t = p_and_t![collected 2, 4];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [-4, -8, 6, 6]);
    }

    #[test]
    fn disorder() {
        let collected = CollectedTricks::from_tricks(Tricks::try_from(9).unwrap());
        let t = [(PlayerId(2), collected)];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [-3, -3, 9, -3]);
    }

    #[test]
    fn disorder_2() {
        let t = [
            (
                PlayerId(2),
                CollectedTricks::from_tricks(Tricks::try_from(6).unwrap()),
            ),
            (
                PlayerId(1),
                CollectedTricks::from_tricks(Tricks::try_from(8).unwrap()),
            ),
        ];
        let scores = Scorable.get_each_player_score(&t).unwrap();
        assert_eq!(scores, [2, 8, -12, 2]);
    }

    #[test]
    fn duplicate() {
        let tricks = CollectedTricks::from_tricks(t!(5));
        let t = [(PlayerId(1), tricks), (PlayerId(1), tricks)];
        Scorable.get_each_player_score(&t).unwrap_err();
    }
}
