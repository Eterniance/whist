use crate::gamemodes::{GameMode, GameSet, ModeParams};

pub enum GameRules {
    Dutch,
    French,
}

pub fn select_rules(rules: GameRules) -> Vec<GameSet> {
    match rules {
        GameRules::Dutch => {
            let emballage = GameSet {
                gamemode: GameMode::Emballage,
                params: ModeParams {
                    player_number: 2,
                    tricks_to_win: 8,
                    min_points: 2,
                    points_per_suppl_trick: 1,
                    max_tricks_allowed: None,
                },
            };
            let seul = GameSet {
                gamemode: GameMode::Seul,
                params: ModeParams {
                    player_number: 1,
                    tricks_to_win: 6,
                    min_points: 6,
                    points_per_suppl_trick: 3,
                    max_tricks_allowed: Some(8),
                },
            };
            vec![emballage, seul]
        }
        GameRules::French => {
            let emballage = GameSet {
                gamemode: GameMode::Emballage,
                params: ModeParams {
                    player_number: 2,
                    tricks_to_win: 8,
                    min_points: 2,
                    points_per_suppl_trick: 2,
                    max_tricks_allowed: None,
                },
            };
            let seul = GameSet {
                gamemode: GameMode::Seul,
                params: ModeParams {
                    player_number: 1,
                    tricks_to_win: 5,
                    min_points: 6,
                    points_per_suppl_trick: 3,
                    max_tricks_allowed: None,
                },
            };
            vec![emballage, seul]
        }
    }
}