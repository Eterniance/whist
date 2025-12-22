use std::collections::HashMap;

use itertools::{Either, Itertools};

use super::GameError;

pub enum Contractors {
    Solo(PlayerId),
    Team(PlayerId, PlayerId),
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerId(usize);

impl PlayerId {
    pub fn new(idx: usize) -> Self {
        Self(idx)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: i16,
    id: PlayerId,
}

impl Player {
    fn new(name: String, idx: usize) -> Self {
        Self {
            name,
            score: 0,
            id: PlayerId(idx),
        }
    }
}

#[derive(Debug, Default)]
pub struct Players {
    players: Vec<Player>,
    next_idx: usize,
    name_to_id: HashMap<String, PlayerId>,
}

impl Players {
    pub fn add_player(&mut self, name: String) -> Result<(), GameError> {
        let player = Player::new(name.clone(), self.next_idx);
        if self.players.len() >= 4 {
            return Err(GameError::TooManyPlayer);
        } else if self.players.contains(&player) {
            return Err(GameError::PlayerAlreadyExists);
        }
        self.name_to_id.insert(name, PlayerId(self.next_idx));
        self.next_idx += 1;
        self.players.push(player);
        Ok(())
    }

    pub fn get_id(&self, name: String) -> Option<PlayerId> {
        self.name_to_id.get(&name).cloned()
    }

    pub fn update_score(&mut self, contractors: Contractors, score: i16) {
        match contractors {
            Contractors::Solo(PlayerId(idx)) => {
                for (i, player) in self.players.iter_mut().enumerate() {
                    if i == idx {
                        player.score += score;
                    } else {
                        player.score -= score / 3;
                    }
                }
            }
            Contractors::Team(PlayerId(idx_1), PlayerId(idx_2)) => {
                let (contractors, others): (Vec<_>, Vec<_>) =
                    self.players.iter_mut().enumerate().partition_map(|(i, p)| {
                        if (i == idx_1) | (i == idx_2) {
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

#[cfg(test)]
mod tests {
    use crate::game::rules::{GameRules, select_rules};

    use super::*;

    #[test]
    fn test_players() {
        let gamemodes = select_rules(GameRules::Dutch);
        let names = ["A", "B", "C", "D"];
        let mut players = Players::default();
        for name in names {
            players.add_player(name.to_string()).unwrap();
        }
        let tricks = 8;

        let score = gamemodes[0].get_score(tricks);
        let contractors = Contractors::Team(
            players.get_id("A".to_string()).unwrap(),
            players.get_id("B".to_string()).unwrap(),
        );
        players.update_score(contractors, score);

        assert_eq!(players.players[0].score, 2);
        assert_eq!(players.players[1].score, 2);
        assert_eq!(players.players[2].score, -2);
        assert_eq!(players.players[3].score, -2);
    }
}
