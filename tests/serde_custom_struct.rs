mod utils;
#[cfg(test)]
#[cfg(feature = "serde")]
mod test {
    use std::error::Error;

    use crate::{t, utils::build_players};
    use whist_game::{CollectedTricks, Contract, Score, Tricks, hand::HandBuilder};

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct Chase4 {}

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct ContractHolder {
        contract: Contract
    }

    #[typetag::serde]
    impl Score for Chase4 {
        fn min_tricks(&self) -> Tricks {
            Tricks::new(0).unwrap()
        }

        fn calculate_score(&self, tricks: CollectedTricks) -> i16 {
            match tricks.absolute.get() {
                4 => 21,
                3 => -15,
                2 => -10,
                1 => -5,
                0 => 0,
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn chase4_smoke() {
        let chase = Chase4 {};

        let tricks = CollectedTricks::from_tricks(t!(2));
        assert_eq!(chase.calculate_score(tricks), -10);
    }

    #[test]
    fn serde_emb() {
        let name = "Queens chase".to_string();
        let gamemode = Chase4 {};
        let contract = Contract {
            name,
            max_bid: None,
            contractors_kind: 1..=3,
            gamemode: Box::new(gamemode),
        };
        let holder = ContractHolder {contract};

        let s = serde_json::to_string(&holder).unwrap();
    let back: ContractHolder = serde_json::from_str(&s).unwrap();

    assert_eq!(back.contract.name, "Queens chase");
    assert_eq!(back.contract.contractors_kind, 1..=3);
    assert_eq!(back.contract.gamemode.min_tricks(), Tricks::new(0).unwrap());
    }

    #[test]
    fn whole_game() -> Result<(), Box<dyn Error>>{
        let name = "Queens chase".to_string();
        let mut players = build_players()?;
        let gamemode = Chase4 {};
        let contract = Contract {
            name: name.clone(),
            max_bid: None,
            contractors_kind: 1..=3,
            gamemode: Box::new(gamemode),
        };

        let mut hb = HandBuilder::new(contract.clone());
        let c1 = players.get_id("Player 3").ok_or("no player")?;
        let c2 = players.get_id("Player 2").ok_or("no player")?;
        let contractors = [c1, c2];
        hb.set_contractors(&contractors)?;
        hb.set_tricks(&[t!(1), t!(3)])?;
        let hand = hb.build()?;
        assert_eq!(hand.gamemode_name(), name);
        let scores = hand.get_score()?;
        let expected_score_1 = [10,-15,-5,10];
        assert_eq!(scores, expected_score_1);
        players.update_score(&scores)?;
        assert_eq!(players.current_scores(), expected_score_1);

        let mut hb = HandBuilder::new(contract);
        let c1 = players.get_id("Player 4").ok_or("no player")?;
        let contractors = [c1];
        hb.set_contractors(&contractors)?;
        hb.set_tricks(&[t!(4)])?;
        let hand = hb.build()?;
        let scores = hand.get_score()?;
        let expected_score_2 = [-7,-7,-7,21];
        assert_eq!(scores, expected_score_2);
        players.update_score(&scores)?;
        assert_eq!(players.current_scores().iter().sum::<i16>(), 0);
        assert_eq!(players.current_scores(), [3,-22,-12,31]);

        Ok(())
    }
}
