use std::error::Error;

use whist_game::{Tricks, default_contracts, hand::HandBuilder};
mod utils;
use crate::utils::build_players;

#[test]
fn main() -> Result<(), Box<dyn Error>> {
    let mut players = build_players()?;
    let contracts = default_contracts();

    let mut hand_builder = HandBuilder::new(contracts[0].clone()); // Emballage

    let c1 = players.get_id("Player 1").ok_or("no player")?;
    let c2 = players.get_id("Player 2").ok_or("no player")?;
    let contractors = [c1, c2];
    hand_builder.set_contractors(&contractors)?;
    hand_builder.set_bid(Tricks::new(9)?)?;
    hand_builder.set_tricks(&[Tricks::new(10)?])?;
    let hand = hand_builder.build()?;
    assert_eq!(hand.gamemode_name(), "Emballage");

    let scores = hand.get_score()?;
    assert_eq!(scores, [3, 3, -3, -3]);
    players.update_score(&scores)?;
    assert_eq!(players.current_scores(), [3, 3, -3, -3]);

    let mut hand_builder = HandBuilder::new(contracts[2].clone()); // Seul

    let c1 = players.get_id("Player 3").ok_or("no player")?;
    let contractors = [c1];
    hand_builder.set_contractors(&contractors)?;
    hand_builder.set_bid(Tricks::new(7)?)?;
    hand_builder.set_tricks(&[Tricks::new(7)?])?;
    let hand = hand_builder.build()?;
    assert_eq!(hand.gamemode_name(), "Seul");

    let scores = hand.get_score()?;
    assert_eq!(scores, [-2, -2, 6, -2]);
    players.update_score(&scores)?;
    assert_eq!(players.current_scores(), [1, 1, 3, -5]);

    Ok(())
}
