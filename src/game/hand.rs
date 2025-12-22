use super::{
    GameError,
    players::Contractors,
    rules::{Contract, ContractorsKind},
};

#[derive(Debug)]
pub struct Hand<'hand> {
    pub contract: &'hand Contract,
    pub contractors: Contractors,
    pub bid: Option<i16>,
}

#[derive(Debug)]
pub enum InputRequest {
    ContractorsSolo,
    ContractorsTeam,
    ContractorsOther,
    Bid { min: i16, max: i16 },
    Done,
}

pub struct HandBuilder<'hand> {
    contract: &'hand Contract,
    contractors: Option<Contractors>,
    bid: Option<i16>,
}

impl<'hand> HandBuilder<'hand> {
    pub fn new(contract: &'hand Contract) -> Self {
        Self {
            contract,
            contractors: None,
            bid: None,
        }
    }

    pub fn next_request(&self) -> InputRequest {
        if self.contractors.is_none() {
            return match self.contract.contractors_kind {
                ContractorsKind::Solo => InputRequest::ContractorsSolo,
                ContractorsKind::Team => InputRequest::ContractorsTeam,
                ContractorsKind::Other => InputRequest::ContractorsOther,
            };
        }
        if self.bid.is_none()
            && let Some(max) = self.contract.max_bid
        {
            let min = self.contract.min_tricks();
            return InputRequest::Bid { min, max };
        };

        InputRequest::Done
    }

    pub fn set_contractors(&mut self, c: Contractors) -> Result<(), GameError> {
        if self.contract.contractors_kind != c {
            return Err(GameError::HandBuildError(
                "Contractors type does not match".to_string(),
            ));
        }

        self.contractors = Some(c);
        Ok(())
    }

    pub fn set_bid(&mut self, bid: i16) -> Result<(), GameError> {
        if let Some(max_bid) = self.contract.max_bid {
            if bid < self.contract.min_tricks() && bid > max_bid {
                return Err(GameError::HandBuildError("Bid out of range".to_string()));
            }
            self.bid = Some(bid);
        } else {
            self.bid = None;
        }
        Ok(())
    }

    pub fn build(self) -> Result<Hand<'hand>, GameError> {
        let contractors = self
            .contractors
            .ok_or(GameError::HandBuildError("No contractors".to_string()))?;
        if self.contract.max_bid.is_some() && self.bid.is_none() {
            return Err(GameError::HandBuildError("Missing bid".to_string()));
        };
        Ok(Hand {
            contract: self.contract,
            contractors,
            bid: self.bid,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{
            players::PlayerId,
            rules::{GameRules, select_rules},
        },
        gamemodes::TOTAL_TRICKS,
    };

    use super::*;

    #[test]
    fn build_hand() {
        let scorables = select_rules(GameRules::Dutch);
        let emballage = &scorables[0];

        let mut builder = HandBuilder::new(emballage);

        assert!(matches!(
            builder.next_request(),
            InputRequest::ContractorsTeam
        ));
        let player1 = PlayerId::new(0);
        let player2 = PlayerId::new(1);
        let c = Contractors::Team(player1.clone(), player2.clone());
        builder.set_contractors(c).unwrap();

        let (min, max) = match builder.next_request() {
            InputRequest::Bid { min, max } => (min, max),
            other => panic!("Expected Bid request, got {:?}", other),
        };
        assert_eq!(min, 8);
        assert_eq!(max, TOTAL_TRICKS);

        let bid = 10;
        assert!(bid <= max);
        builder.set_bid(bid).unwrap();

        assert!(matches!(builder.next_request(), InputRequest::Done));

        let hand = builder.build().unwrap();

        match hand.contractors {
            Contractors::Team(a, b) => {
                // Keep these checks aligned with the `c` above.
                assert_eq!(a, player1);
                assert_eq!(b, player2);
            }
            _ => panic!("Expected Contractors::Team"),
        }

        assert_eq!(hand.bid, Some(bid));
    }

    #[test]
    fn build_hand_picolo() {
        let scorables = select_rules(GameRules::French);
        let picolo = &scorables[2];

        let mut builder = HandBuilder::new(picolo);

        assert!(matches!(
            builder.next_request(),
            InputRequest::ContractorsSolo
        ));

        let player = PlayerId::new(0);
        builder
            .set_contractors(Contractors::Solo(player.clone()))
            .unwrap();

        let bid = 4;
        builder.set_bid(bid).unwrap();

        assert!(builder.bid.is_none());

        assert!(matches!(builder.next_request(), InputRequest::Done));

        let hand = builder.build().unwrap();

        match hand.contractors {
            Contractors::Solo(p) => assert_eq!(p, player),
            _ => panic!("Expected Contractors::Solo"),
        }

        assert!(hand.bid.is_none());
    }

    #[test]
    fn build_hand_failures() {
        let scorables = select_rules(GameRules::Dutch);
        let emballage= &scorables[0]; // Emballage: Team + bid required

        let builder = HandBuilder::new(emballage);
        let err = builder.build().unwrap_err();
        assert!(matches!(err, GameError::HandBuildError(_)));

        let mut builder = HandBuilder::new(emballage);

        let solo_player = PlayerId::new(0);
        let err = builder
            .set_contractors(Contractors::Solo(solo_player))
            .unwrap_err();

        assert!(matches!(err, GameError::HandBuildError(_)));

        let mut builder = HandBuilder::new(emballage);

        let p1 = PlayerId::new(0);
        let p2 = PlayerId::new(1);
        builder.set_contractors(Contractors::Team(p1, p2)).unwrap();

        assert!(matches!(builder.next_request(), InputRequest::Bid { .. }));

        let err = builder.build().unwrap_err();
        assert!(matches!(err, GameError::HandBuildError(_)));
    }
}
