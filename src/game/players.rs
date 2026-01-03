use std::collections::HashMap;

use itertools::{Either, Itertools};

use crate::game::{hand::Requester, rules::ContractorsKind};

use super::GameError;

#[derive(Debug, Clone)]
#[repr(usize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Contractors {
    Solo(PlayerId) = 1,
    Team(PlayerId, PlayerId) = 2,
    Other = 4,
}

impl PartialEq<ContractorsKind> for Contractors {
    fn eq(&self, other: &ContractorsKind) -> bool {
        other == self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerId(usize);

impl PlayerId {
    #[must_use]
    pub const fn new(idx: usize) -> Self {
        Self(idx)
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

    pub fn update_score(&mut self, contractors: &Contractors, score: i16) {
        match contractors {
            Contractors::Solo(PlayerId(idx)) => {
                for (i, player) in self.list.iter_mut().enumerate() {
                    if i == *idx {
                        player.score += score;
                    } else {
                        player.score -= score / 3;
                    }
                }
            }
            Contractors::Team(PlayerId(idx_1), PlayerId(idx_2)) => {
                let (contractors, others): (Vec<_>, Vec<_>) =
                    self.list.iter_mut().enumerate().partition_map(|(i, p)| {
                        if (i == *idx_1) | (i == *idx_2) {
                            Either::Left(p)
                        } else {
                            Either::Right(p)
                        }
                    });

                for (contractor, other) in contractors.into_iter().zip(others) {
                    contractor.score += score;
                    other.score -= score;
                }
            }
            Contractors::Other => todo!(),
        }
    }
}

pub async fn fill_players<R, F>(mut players: Players, req: &mut R, on_duplicate: F) -> Players
where
    R: Requester,
    F: Fn(),
{
    loop {
        let Ok(name) = req.ask_name().await else {
            continue;
        };
        match players.add_player(name) {
            Ok(4) => {
                break;
            }
            Err(GameError::PlayerAlreadyExists) => {
                on_duplicate();
            }
            _ => {}
        }
    }
    players
}

#[cfg(test)]
mod tests {
    use crate::game::rules::{GameRules, select_rules};
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

        let score = contracts[0].gamemode.get_score(tricks);
        let contractors =
            Contractors::Team(players.get_id("A").unwrap(), players.get_id("B").unwrap());
        players.update_score(&contractors, score);

        assert_eq!(players.list[0].score, 2);
        assert_eq!(players.list[1].score, 2);
        assert_eq!(players.list[2].score, -2);
        assert_eq!(players.list[3].score, -2);
    }
}
