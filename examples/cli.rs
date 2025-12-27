use std::{io::Error, sync::Arc};
use whist::{
    game::{
        hand::build_hand,
        players::Players,
        rules::{GameRules, select_rules},
    },
    gamemodes::Score,
};
mod cli_requester;
use cli_requester::CliRequest;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let local = tokio::task::LocalSet::new();

    let rules: Vec<_> = select_rules(&GameRules::Dutch)
        .into_iter()
        .map(Arc::new)
        .collect();
    let cli = CliRequest::new();
    let mut players = Arc::new(Players::from_list(&["A", "B", "C", "D"]).unwrap());

    let (tx, rx) = tokio::sync::oneshot::channel();

    let players_cp = Arc::clone(&players);
    local
        .run_until(async move {
            tokio::task::spawn_local(async move {
                let hand = build_hand(rules[1].clone(), players_cp, Arc::new(cli))
                    .await
                    .unwrap();
                let _ = tx.send(hand);
            })
            .await
            .unwrap();
        })
        .await;

    let hand = rx.await.unwrap();

    println!("hand is: {hand:?}");
    let tricks = 7;
    let score = hand.get_score(tricks);
    println!("points received: {score}");

    Arc::get_mut(&mut players)
        .unwrap()
        .update_score(&hand.contractors, score);

    println!("finally: {players:?}");

    Ok(())
}
