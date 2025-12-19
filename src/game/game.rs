use std::collections::HashMap;

pub enum Contractors<'hand> {
    Solo(&'hand str),
    Team(&'hand str, &'hand str),
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: i16,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self { name, score: 0 }
    }
}

pub struct Players {
    pub players: HashMap<String, Player>,
}

impl Players {
    pub fn new(players: [Player; 4]) -> Self {
        let mut hmap = HashMap::with_capacity(4);
        for p in players {
            hmap.insert(p.name.clone(), p);
        }
        Self { players: hmap }
    }

    pub fn update_score(&mut self, contractors: Contractors, score: i16) {
        match contractors {
            Contractors::Solo(contractor_name) => {
                for (name, p) in self.players.iter_mut() {
                    if name == &contractor_name {
                        p.score += score;
                    } else {
                        p.score -= score / 3;
                    }
                }
            }
            Contractors::Team(contractor_name1, contractor_name2) => {
                let (contractors, others): (Vec<_>, Vec<_>) =
                    self.players.iter_mut().map(|(_, p)| p).partition(|p| {
                        ((**p).name == contractor_name1) | ((**p).name == contractor_name2)
                    });

                for (contractor, other) in contractors.into_iter().zip(others) {
                    contractor.score += score;
                    other.score -= score;
                }
            }
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
        let a = Player::new("A".to_string());
        let b = Player::new("B".to_string());
        let c = Player::new("C".to_string());
        let d = Player::new("D".to_string());
        let mut players = Players::new([a, b, c, d]);

        let tricks = 8;

        let score = gamemodes[0].get_score(tricks);

        players.update_score(Contractors::Team("A", "B"), score);

        assert_eq!(players.players.get("A").unwrap().score, 2);
        assert_eq!(players.players.get("B").unwrap().score, 2);
        assert_eq!(players.players.get("C").unwrap().score, -2);
        assert_eq!(players.players.get("D").unwrap().score, -2);
    }
}
