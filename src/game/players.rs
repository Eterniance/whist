use std::collections::HashMap;

use itertools::Itertools;

use super::GameError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerId(pub(crate) usize);

impl PlayerId {
    #[must_use]
    pub const fn new(idx: usize) -> Self {
        Self(idx)
    }

    #[must_use]
    pub const fn idx(&self) -> usize {
        self.0
    }
}

impl PartialEq<PlayerIdAndScore> for PlayerId {
    fn eq(&self, other: &PlayerIdAndScore) -> bool {
        other == self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerIdAndScore {
    pub id: PlayerId,
    pub score: i16,
}

impl PlayerIdAndScore {
    #[must_use]
    pub const fn new(id: PlayerId, score: i16) -> Self {
        Self { id, score }
    }

    #[must_use]
    pub const fn as_components(&self) -> (&usize, &i16) {
        (&self.id.0, &self.score)
    }

    #[must_use]
    pub const fn from_id(id: PlayerId) -> Self {
        Self::new(id, 0)
    }
}

impl PartialEq<PlayerId> for PlayerIdAndScore {
    fn eq(&self, other: &PlayerId) -> bool {
        self.id == *other
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Player {
    pub name: String,
    pub score: i16,
    id: PlayerId,
}

impl Player {
    const fn new(name: String, idx: usize) -> Self {
        Self {
            name,
            score: 0,
            id: PlayerId(idx),
        }
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Players {
    pub list: Vec<Player>,
    next_idx: usize,
    name_to_id: HashMap<String, PlayerId>,
}

impl Players {
    /// Creates a `Players` collection from a fixed list of four player names.
    ///
    /// The provided values are converted to strings and added in order using the
    /// standard validation rules.
    ///
    /// # Errors
    ///
    /// Returns an error if adding any player fails, for example if a name is
    /// duplicated or if the maximum number of players is exceeded.
    pub fn from_list(names: &[impl std::string::ToString; 4]) -> Result<Self, GameError> {
        let mut players = Self::default();
        for n in names {
            players.add_player(n.to_string())?;
        }
        Ok(players)
    }

    /// Adds a new player to the game.
    ///
    /// A player is created with the given name and assigned a unique internal
    /// identifier. The game must contain four players. Returns the number of players.
    ///
    /// # Errors
    ///
    /// Returns an `GameError` if the game already has four players or if the player
    /// already exists.
    pub fn add_player(&mut self, name: String) -> Result<usize, GameError> {
        let player = Player::new(name.clone(), self.next_idx);
        if self.list.len() >= 4 {
            return Err(GameError::TooManyPlayer);
        } else if self.name_to_id.keys().contains(&name) {
            return Err(GameError::PlayerAlreadyExists);
        }
        self.name_to_id.insert(name, PlayerId(self.next_idx));
        self.next_idx += 1;
        self.list.push(player);
        Ok(self.list.len())
    }

    #[must_use]
    pub fn get_id(&self, name: &str) -> Option<PlayerId> {
        self.name_to_id.get(name).cloned()
    }

    #[must_use]
    pub fn names(&self) -> Vec<String> {
        self.list.iter().map(|p| p.name.clone()).collect()
    }

    /// Update each player score.
    ///
    /// Each contractor score will be increase by `score` and the others score will be decrease
    /// to keep the score sum null.
    /// Note that score can be negative.
    ///
    /// # Errors
    /// This function can only fail if `Contractors == Contractors::Other` in the event
    /// that the score sum is not zero
    /// (for example, an impossible amount of players has been supplied).
    #[allow(clippy::missing_panics_doc)]
    pub fn update_score(&mut self, scores: &[i16; 4]) {
        for (player, score) in self.list.iter_mut().zip(scores.iter()) {
            player.score += score;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::contractors::ContractorsScore;
    use crate::game::hand::InputError;
    use crate::game::rules::{GameRules, calculate_players_score, select_rules};
    use crate::gamemodes::Score;

    use super::*;

    #[test]
    fn test_players() {
        let contracts = select_rules(&GameRules::Dutch);
        let names = ["A", "B", "C", "D"];
        let mut players = Players::default();
        for name in names {
            players.add_player(name.to_string()).unwrap();
        }
        let tricks = 8;

        let score = contracts[0].gamemode.get_single_player_score(tricks);
        let contractors = ContractorsScore::Team(
            PlayerIdAndScore::new(players.get_id("A").unwrap(), score),
            PlayerIdAndScore::new(players.get_id("B").unwrap(), score),
        );
        let scores = calculate_players_score(contractors).unwrap();
        players.update_score(&scores);

        assert_eq!(players.list[0].score, 2);
        assert_eq!(players.list[1].score, 2);
        assert_eq!(players.list[2].score, -2);
        assert_eq!(players.list[3].score, -2);
    }
    #[test]
    fn update_score_solo_wrong_score_fails() {
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        // Not divisible by 3 => WrongScore
        let contractors =
            ContractorsScore::Solo(PlayerIdAndScore::new(players.get_id("A").unwrap(), 5));

        let err = calculate_players_score(contractors).unwrap_err();
        assert!(matches!(err, InputError::WrongScore));
    }

    #[test]
    fn update_score_team_ok() {
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        // Team requires (score_1 + score_2) % 2 == 0
        let contractors = ContractorsScore::Team(
            PlayerIdAndScore::new(players.get_id("A").unwrap(), 4),
            PlayerIdAndScore::new(players.get_id("B").unwrap(), 2),
        );
        let scores = calculate_players_score(contractors).unwrap();
        players.update_score(&scores);

        // Others get -(4+2)/2 = -3
        assert_eq!(players.list[0].score, 4);
        assert_eq!(players.list[1].score, 2);
        assert_eq!(players.list[2].score, -3);
        assert_eq!(players.list[3].score, -3);
    }

    #[test]
    fn update_score_team_same_player_fails() {
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        let a = players.get_id("A").unwrap();

        let contractors = ContractorsScore::Team(
            PlayerIdAndScore::new(a.clone(), 2),
            PlayerIdAndScore::new(a, 2),
        );

        let err = calculate_players_score(contractors).unwrap_err();
        assert!(matches!(err, InputError::InvalidInput(_)));
    }

    #[test]
    fn update_score_team_wrong_score_fails() {
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        // 3 + 2 = 5 (odd) => WrongScore
        let contractors = ContractorsScore::Team(
            PlayerIdAndScore::new(players.get_id("A").unwrap(), 3),
            PlayerIdAndScore::new(players.get_id("B").unwrap(), 2),
        );

        let err = calculate_players_score(contractors).unwrap_err();
        assert!(matches!(err, InputError::WrongScore));
    }

    #[test]
    fn update_score_other_len1_delegates_to_solo() {
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        let contractors =
            ContractorsScore::Other(vec![PlayerIdAndScore::new(players.get_id("A").unwrap(), 6)]);
        let scores = calculate_players_score(contractors).unwrap();
        players.update_score(&scores);

        assert_eq!(players.list[0].score, 6);
        assert_eq!(players.list[1].score, -2);
        assert_eq!(players.list[2].score, -2);
        assert_eq!(players.list[3].score, -2);
    }

    #[test]
    fn update_score_other_len2_delegates_to_team() {
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        let contractors = ContractorsScore::Other(vec![
            PlayerIdAndScore::new(players.get_id("A").unwrap(), -5),
            PlayerIdAndScore::new(players.get_id("B").unwrap(), -15),
        ]);
        let scores = calculate_players_score(contractors).unwrap();
        players.update_score(&scores);

        assert_eq!(players.list[0].score, -5);
        assert_eq!(players.list[1].score, -15);
        assert_eq!(players.list[2].score, 10);
        assert_eq!(players.list[3].score, 10);
    }

    #[test]
    fn update_score_other_len3_ok() {
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        // For the len=3 branch, your implementation computes the "missing" player index
        // and applies the negated accumulated score to them.
        // Choose scores that sum to 0 so the last player stays unchanged.
        let contractors = ContractorsScore::Other(vec![
            PlayerIdAndScore::new(players.get_id("A").unwrap(), 2),
            PlayerIdAndScore::new(players.get_id("B").unwrap(), -1),
            PlayerIdAndScore::new(players.get_id("C").unwrap(), -1),
        ]);
        let scores = calculate_players_score(contractors).unwrap();
        players.update_score(&scores);

        assert_eq!(players.list[0].score, 2);
        assert_eq!(players.list[1].score, -1);
        assert_eq!(players.list[2].score, -1);
        assert_eq!(players.list[3].score, 0);
    }

    #[test]
    fn update_score_other_wrong_len_fails() {
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        // len=4 is not handled => WrongScore
        let contractors = ContractorsScore::Other(vec![
            PlayerIdAndScore::new(players.get_id("A").unwrap(), 1),
            PlayerIdAndScore::new(players.get_id("B").unwrap(), 1),
            PlayerIdAndScore::new(players.get_id("C").unwrap(), 1),
            PlayerIdAndScore::new(players.get_id("D").unwrap(), 1),
        ]);

        let err = calculate_players_score(contractors).unwrap_err();
        assert!(matches!(err, InputError::WrongScore));
    }
}
