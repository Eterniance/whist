use std::error::Error;

use whist_game::{Tricks, default_contracts, hand::HandBuilder};

mod utils;
use crate::utils::build_players;

fn run_case(
    players: &mut whist_game::players::Players,
    contract: whist_game::Contract,
    contractor_names: &[&str],
    tricks: &[Tricks],
    expected_scores: [i16; 4],
) -> Result<(), Box<dyn Error>> {
    let mut hb = HandBuilder::new(contract);

    let mut ids = Vec::with_capacity(contractor_names.len());
    for &name in contractor_names {
        ids.push(players.get_id(name).ok_or("no player")?);
    }

    hb.set_contractors(&ids)?;
    hb.set_tricks(tricks)?;
    let hand = hb.build()?;

    assert_eq!(hand.gamemode_name(), "Petite Misere");

    let scores = hand.get_score()?;
    assert_eq!(scores, expected_scores);

    players.update_score(&scores)?;
    Ok(())
}

#[test]
fn petite_misere_contractors_kind_1_2_3() -> Result<(), Box<dyn Error>> {
    let mut players = build_players()?;
    let contracts = default_contracts();

    let petite = contracts
        .iter()
        .find(|c| c.name == "Petite Misere")
        .ok_or("Petite Misere not found")?
        .clone();

    run_case(
        &mut players,
        petite.clone(),
        &["Player 1"],
        &[Tricks::new(0)?],
        [12, -4, -4, -4],
    )?;
    assert_eq!(players.current_scores(), [12, -4, -4, -4]);

    run_case(
        &mut players,
        petite.clone(),
        &["Player 2", "Player 3"],
        &[Tricks::new(0)?, Tricks::new(0)?],
        [-12, 12, 12, -12],
    )?;
    assert_eq!(players.current_scores(), [0, 8, 8, -16]);

    let mut hb = HandBuilder::new(petite);
    let c1 = players.get_id("Player 1").ok_or("no player")?;
    let c2 = players.get_id("Player 4").ok_or("no player")?;
    let c3 = players.get_id("Player 3").ok_or("no player")?;
    let contractors = [c1, c2, c3];
    hb.set_contractors(&contractors)?;
    hb.set_tricks(&[Tricks::new(1)?, Tricks::new(0)?, Tricks::new(0)?])?;
    let hand = hb.build()?;

    assert_eq!(hand.gamemode_name(), "Petite Misere");

    let scores = hand.get_score()?;
    assert_eq!(scores, [-24, 0, 12, 12]);

    players.update_score(&scores)?;
    assert_eq!(players.current_scores(), [-24, 8, 20, -4]);

    Ok(())
}
