use std::ops::{Div, Neg};

use crate::{
    CollectedTricks, TOTAL_PLAYERS,
    players::PlayerId,
    scoring::{Score, Tricks},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExactTricks {
    min_points: i16,
    target: Tricks,
}

impl ExactTricks {
    #[must_use]
    pub const fn new(min_points: i16, target: Tricks) -> Self {
        Self { min_points, target }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Score for ExactTricks {
    fn calculate_score(&self, tricks: CollectedTricks) -> i16 {
        if tricks.absolute == self.target {
            return self.min_points;
        }
        -2 * self.min_points
    }

    fn min_tricks(&self) -> Tricks {
        Tricks::new(0).expect("Within range")
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExactTricksDuo(ExactTricks);

impl ExactTricksDuo {
    #[must_use]
    pub const fn new(inner: ExactTricks) -> Self {
        Self(inner)
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Score for ExactTricksDuo {
    fn calculate_score(&self, tricks: CollectedTricks) -> i16 {
        if tricks.absolute == self.0.target {
            return self.0.min_points;
        }
        -self.0.min_points
    }

    fn min_tricks(&self) -> Tricks {
        self.0.min_tricks()
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn get_each_player_score(
        &self,
        players_and_tricks: &[(PlayerId, CollectedTricks)],
    ) -> Result<[i16; TOTAL_PLAYERS], Box<dyn std::error::Error>> {
        if players_and_tricks.len() != 2 {
            return Err("Only two players case is legal".into());
        }
        let mut winners_num = 0;
        for (_, c) in players_and_tricks {
            if c.absolute == self.0.target {
                winners_num += 1;
            }
        }
        let multiplier = if winners_num == 2 { 1 } else { 2 };
        let mut scores = [0; TOTAL_PLAYERS];
        let mut already_set_mask = 0;
        for (id, tricks) in players_and_tricks {
            let idx = id.idx();
            let already_set_bit = 1 << idx;
            if (already_set_mask & already_set_bit) != 0 {
                return Err("Duplicate PlayerId".into());
            }
            already_set_mask |= already_set_bit;

            scores[idx] = self.calculate_score(*tricks) * multiplier;
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
mod tests {
    use crate::p_and_t;

    use super::*;

    const MISERE: ExactTricks = ExactTricks {
        min_points: 12,
        target: Tricks(0),
    };

    const MISERE_DUO: ExactTricksDuo = ExactTricksDuo(MISERE);

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

    #[test]
    fn duo_win_both() {
        let tricks = p_and_t!(collected 0,0);
        let expected_score = [12, 12, -12, -12];
        let score = MISERE_DUO.get_each_player_score(&tricks).unwrap();

        assert_eq!(score, expected_score);
    }

    #[test]
    fn duo_win_single() {
        let tricks = p_and_t!(collected 0, 1);
        let expected_score = [24, -24, 0, 0];
        let score = MISERE_DUO.get_each_player_score(&tricks).unwrap();
        assert_eq!(score, expected_score);

        let tricks = p_and_t!(collected 1, 0);
        let expected_score = [-24, 24, 0, 0];
        let score = MISERE_DUO.get_each_player_score(&tricks).unwrap();
        assert_eq!(score, expected_score);
    }

    #[test]
    fn duo_lose_both() {
        let tricks = p_and_t!(collected 1,4);
        let expected_score = [-24, -24, 24, 24];
        let score = MISERE_DUO.get_each_player_score(&tricks).unwrap();

        assert_eq!(score, expected_score);
    }

    #[test]
    fn duo_error_on_wrong_input() {
        let tricks = p_and_t!(collected 1);
        let _ = MISERE_DUO.get_each_player_score(&tricks).unwrap_err();
        let tricks = p_and_t!(collected 1, 1, 0);
        let _ = MISERE_DUO.get_each_player_score(&tricks).unwrap_err();
    }
}
