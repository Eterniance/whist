use crate::{GameError, contracts::hand::InputError};
use itertools::Itertools;
use std::collections::HashMap;

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
    /// The player score in position `self.list[i]` will be incremented by `scores[i]`
    /// # Errors
    /// This function returns early with error if the sum of every elements in `scores`
    /// is non zero.
    pub fn update_score(&mut self, scores: &[i16; 4]) -> Result<(), InputError> {
        if scores.iter().sum::<i16>() != 0 {
            return Err(InputError::WrongScore);
        }
        for (player, score) in self.list.iter_mut().zip(scores.iter()) {
            player.score += score;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{PlayerId, Players};
    use crate::{contracts::default_contracts, p_and_t, scoring::Tricks};

    #[test]
    fn test_players() {
        let contracts = default_contracts();
        let mut players = Players::from_list(&["A", "B", "C", "D"]).unwrap();

        let p_and_t = p_and_t!(8, 8);
        let scores = contracts[0]
            .gamemode
            .get_each_player_score(&p_and_t)
            .unwrap();
        players.update_score(&scores).unwrap();

        assert_eq!(players.list[0].score, 2);
        assert_eq!(players.list[1].score, 2);
        assert_eq!(players.list[2].score, -2);
        assert_eq!(players.list[3].score, -2);
    }
}
