#[macro_export]
macro_rules! p {
        ( $($idx:literal),+ $(,)? ) => {
            [$(whist_game::players::PlayerId($idx)),+]
        };
    }

#[macro_export]
macro_rules! t {
    ($v:literal) => {
        whist_game::Tricks::new($v).unwrap()
    };
}

#[macro_export]
macro_rules! p_and_t {
    (collected $($trick:literal),+ $(,)? ) => {
        {let mut idx = 0;
            #[allow(unused)]
            [ $(
                {
                    let collected = whist_game::scoring::CollectedTricks::from_tricks(whist_game::t!($trick));
                    let pair = (whist_game::players::PlayerId(idx), collected);
                    idx += 1;
                    pair
                },
            )+]
        }
    };
    ( $($trick:literal),+ $(,)? ) => {
        {let mut idx = 0;
            #[allow(unused)]
            [ $(
                {
                    let pair = (whist_game::players::PlayerId(idx), whist_game::t!($trick));
                    idx += 1;
                    pair
                },
            )+]
        }
    };
}

#[allow(clippy::missing_errors_doc, dead_code)]
pub fn build_players() -> Result<whist_game::players::Players, Box<dyn std::error::Error>> {
    let mut players_builder = whist_game::players::PlayersBuilder::new();
    players_builder.add_player(&"Player 1")?;
    players_builder.add_player(&"Player 2")?;
    players_builder.add_player(&"Player 3")?;
    players_builder.add_player(&"Player 4")?;
    Ok(players_builder.build()?)
}
