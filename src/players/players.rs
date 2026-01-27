use crate::{GameError, TOTAL_PLAYERS, contracts::hand::InputError};
use itertools::Itertools;
use std::collections::HashMap;

/// Unique player ID.
///
/// This corresponds to the player arrival order.
///
/// # Example
/// ```
/// use whist::players::PlayersBuilder;
///
/// let mut players_builder = PlayersBuilder::default();
/// for p in ["A", "B", "C", "D"].into_iter() {
///     players_builder.add_player(&p).unwrap();
/// }
/// let mut players = players_builder.build().unwrap();
/// let id = players.get_id("A").unwrap();
/// assert_eq!(id.idx(), 0);
/// let id = players.get_id("C").unwrap();
/// assert_eq!(id.idx(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerId(pub(crate) usize);

impl PlayerId {
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

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayersBuilder {
    players: Vec<Player>,
    next_idx: usize,
    name_to_id: HashMap<String, PlayerId>,
}

impl PlayersBuilder {
    /// Adds a new player to the game.
    ///
    /// A player is created with the given name and assigned a unique internal
    /// identifier. The game must contain four players. Returns the number of players.
    ///
    /// # Errors
    ///
    /// Returns an `GameError` if the game already has four players or if the player
    /// already exists.
    pub fn add_player(&mut self, name: &impl ToString) -> Result<usize, GameError> {
        if self.players.len() >= TOTAL_PLAYERS {
            return Err(GameError::PlayersNotSet(self.players.len()));
        }
        if self.name_to_id.keys().contains(&name.to_string()) {
            return Err(GameError::PlayerAlreadyExists);
        }
        let player = Player::new(name.to_string(), self.next_idx);
        self.name_to_id
            .insert(name.to_string(), PlayerId(self.next_idx));
        self.next_idx += 1;
        self.players.push(player);
        Ok(self.players.len())
    }

    /// Consume the builder into `Players`
    ///
    /// # Errors
    /// This function fails if there are not `TOTAL_PLAYERS` players set.
    pub fn build(self) -> Result<Players, GameError> {
        let list = self
            .players
            .try_into()
            .map_err(|v: Vec<_>| GameError::PlayersNotSet(v.len()))?;
        Ok(Players {
            list,
            name_to_id: self.name_to_id,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Players {
    pub list: [Player; TOTAL_PLAYERS],
    name_to_id: HashMap<String, PlayerId>,
}

impl Players {
    #[must_use]
    pub fn get_id(&self, name: &str) -> Option<PlayerId> {
        self.name_to_id.get(name).copied()
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
    pub fn update_score(&mut self, scores: &[i16; TOTAL_PLAYERS]) -> Result<(), InputError> {
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
    use super::PlayersBuilder;
    use crate::{GameError, contracts::default_contracts, p_and_t};

    #[test]
    fn players_builder() -> Result<(), GameError> {
        let contracts = default_contracts();
        let mut players_builder = PlayersBuilder::default();
        for (i, p) in ["A", "B", "C", "D"].into_iter().enumerate() {
            let u = players_builder.add_player(&p)?;
            assert_eq!(u, i + 1);
        }
        let mut players = players_builder.build()?;

        let p_and_t = p_and_t!(collected 8, 8);
        let scores = contracts[0]
            .gamemode
            .get_each_player_score(&p_and_t)
            .unwrap();
        players.update_score(&scores).unwrap();

        assert_eq!(players.list[0].score, 2);
        assert_eq!(players.list[1].score, 2);
        assert_eq!(players.list[2].score, -2);
        assert_eq!(players.list[3].score, -2);
        Ok(())
    }
}
