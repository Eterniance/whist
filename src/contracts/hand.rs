use crate::{TOTAL_PLAYERS, Tricks, contracts::Contract, players::PlayerId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
    #[error("The score sum cannot be zero")]
    WrongScore,
}

#[derive(Debug)]
pub struct Hand {
    contract: Contract,
    contractors_tricks: Vec<(PlayerId, Tricks)>,
    bid: Option<Tricks>,
}

impl Hand {
    #[must_use]
    pub fn gamemode_name(&self) -> String {
        self.contract.name.clone()
    }

    /// Compute the score for each player. The array position corresponds to the player ID.
    ///
    /// # Errors
    /// Returns an error if the remaining score cannot be evenly distributed among the unspecified
    /// players (i.e., the total score is not divisible by the number of missing players).
    ///
    /// # Panics
    /// - Panics if a `PlayerId` index is out of bounds (expected to be `0..4`).
    /// - Panics if internal invariants are violated (final score sum is not zero).
    pub fn get_scores(&self) -> Result<[i16; TOTAL_PLAYERS], Box<dyn std::error::Error>> {
        self.contract.get_scores(&self.contractors_tricks, self.bid)
    }

    #[must_use]
    pub fn as_recap(self, scores: [i16; TOTAL_PLAYERS]) -> HandRecap {
        HandRecap {
            scores,
            gamemode_name: self.gamemode_name(),
            contractors_tricks: self.contractors_tricks,
            bid: self.bid,
        }
    }
}

#[derive(Debug)]
pub enum InputRequest {
    PlayersNumber { min: u8, max: u8 },
    Bid { min: Tricks, max: Tricks },
    Done,
    Cancel,
}

#[derive(Debug)]
pub struct HandBuilder {
    contract: Contract,
    contractors: Option<Vec<PlayerId>>,
    tricks: Option<Vec<Tricks>>,
    bid: Option<Tricks>,
}

impl HandBuilder {
    #[must_use]
    pub const fn new(contract: Contract) -> Self {
        Self {
            contract,
            contractors: None,
            bid: None,
            tricks: None,
        }
    }

    #[must_use]
    pub fn contract_name(&self) -> String {
        self.contract.name.clone()
    }

    #[must_use]
    pub fn all_requests(&self) -> Vec<InputRequest> {
        let mut requests = vec![self.contract_request()];

        if let Some(req) = self.bid_request() {
            requests.push(req);
        }
        requests
    }

    #[must_use]
    pub fn next_request(&self) -> InputRequest {
        if self.contractors.is_none() {
            return self.contract_request();
        }
        if self.bid.is_none()
            && let Some(req) = self.bid_request()
        {
            return req;
        }
        InputRequest::Done
    }

    const fn contract_request(&self) -> InputRequest {
        let (&min, &max) = (
            self.contract.contractors_kind.start(),
            self.contract.contractors_kind.end(),
        );
        InputRequest::PlayersNumber { min, max }
    }

    fn bid_request(&self) -> Option<InputRequest> {
        self.contract.max_bid.map(|max| {
            let min = self.contract.min_tricks();
            InputRequest::Bid { min, max }
        })
    }

    /// Sets the contractors for the current contract.
    ///
    /// The provided contractors must match the contractors type expected by the
    /// current contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the contractors type does not match the contract
    /// configuration.
    #[allow(clippy::missing_panics_doc)]
    pub fn set_contractors(&mut self, c: &[PlayerId]) -> Result<(), HandBuildError> {
        if !self
            .contract
            .contractors_kind
            .contains(&u8::try_from(c.len()).expect("Only 4 players max"))
        {
            return Err(HandBuildError("Contractors type does not match"));
        }
        self.contractors = Some(c.to_vec());
        Ok(())
    }

    /// Sets the bid for the current contract.
    ///
    /// The bid must be within the range allowed by the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the bid is outside the valid range defined by the
    /// contract.
    pub fn set_bid(&mut self, bid: Tricks) -> Result<(), HandBuildError> {
        if let Some(max_bid) = self.contract.max_bid {
            if !(self.contract.min_tricks()..=max_bid).contains(&bid) {
                return Err(HandBuildError("Bid out of range"));
            }
            self.bid = Some(bid);
        } else {
            self.bid = None;
        }
        Ok(())
    }
    /// Sets the number of tricks taken by the contractors.
    ///
    /// The number of provided trick values must match the number of contractors
    /// defined for the hand.
    ///
    /// The `Tricks` position in the slice must correspond to the `PlayerId`
    /// position in `self.contractors`.
    ///
    /// # Errors
    ///
    /// Returns an error if the number of trick entries does not match the
    /// number of contractors.
    #[allow(clippy::missing_panics_doc)]
    pub fn set_tricks(&mut self, tricks: &[Tricks]) -> Result<(), HandBuildError> {
        if self.contractors.is_none() {
            return Err(HandBuildError("Contractors must be set before Tricks"));
        }
        if self.contract.contractors_kind.len() != tricks.len()
            && tricks.len() != 1
            && self
                .contractors
                .as_ref()
                .expect("Checked that is not None")
                .len()
                != tricks.len()
        {
            return Err(HandBuildError(
                "tricks length should be 1 or the same as contractors length",
            ));
        }
        self.tricks = Some(tricks.into());
        Ok(())
    }

    /// Builds the hand from the collected contract parameters.
    ///
    /// All required components must be set before building the hand.
    ///
    /// # Errors
    ///
    /// Returns an error if the contractors are missing, or if a bid is required
    /// by the contract but has not been set.
    #[allow(clippy::missing_panics_doc)]
    pub fn build(self) -> Result<Hand, HandBuildError> {
        let contractors = self.contractors.ok_or(HandBuildError("No contractors"))?;
        let tricks = self.tricks.ok_or(HandBuildError("No tricks set"))?;
        let contractors_tricks = if tricks.len() == 1 {
            contractors
                .into_iter()
                .map(|id| (id, *tricks.first().expect("only one element")))
                .collect()
        } else if tricks.len() == contractors.len() {
            contractors.into_iter().zip(tricks).collect()
        } else {
            return Err(HandBuildError("Incompatible contractors and number tricks"));
        };
        if self.contract.max_bid.is_some() && self.bid.is_none() {
            return Err(HandBuildError("Missing bid"));
        }

        Ok(Hand {
            contract: self.contract,
            contractors_tricks,
            bid: self.bid,
        })
    }
}

#[derive(Debug, Error, Clone, Eq, PartialEq)]
#[error("{0}")]
pub struct HandBuildError(pub &'static str);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HandRecap {
    pub scores: [i16; TOTAL_PLAYERS],
    pub gamemode_name: String,
    pub contractors_tricks: Vec<(PlayerId, Tricks)>,
    pub bid: Option<Tricks>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gamemodes::TricksChase, p, p_and_t, scoring::ExactTricks, t};

    fn emballage() -> Contract {
        let rules = TricksChase::new(t!(8), 2, 1);
        Contract {
            name: "Emballage".to_string(),
            max_bid: Some(Tricks::MAX_TRICKS),
            gamemode: Box::new(rules),
            contractors_kind: 2..=2,
        }
    }

    fn misere() -> Contract {
        let rules = ExactTricks::new(12, Tricks(0));
        Contract {
            name: "Misere".to_string(),
            max_bid: None,
            gamemode: Box::new(rules),
            contractors_kind: 1..=3,
        }
    }

    #[test]
    fn hand_builder_ok() {
        let contract = emballage();

        let contractors = p!(1, 2);
        let bid = t!(9);
        let tricks = &[t!(10)];

        let mut hb = HandBuilder::new(contract);

        hb.set_contractors(&contractors).unwrap();
        hb.set_bid(bid).unwrap();
        hb.set_tricks(tricks).unwrap();

        let hand = hb.build().unwrap();

        assert_eq!(
            hand.contractors_tricks,
            [(PlayerId(1), t!(10)), (PlayerId(2), t!(10))]
        );
        assert_eq!(hand.bid, Some(bid));

        let scores = hand.get_scores().unwrap();
        assert_eq!(scores, [-3, 3, 3, -3]);
    }

    #[test]
    fn hand_builder_fails() {
        let contract = emballage();

        let contractors = p!(1, 2, 3);
        let bid = t!(7);
        let tricks = &[t!(10)];

        let mut hb = HandBuilder::new(contract);

        hb.set_contractors(&contractors).unwrap_err();

        hb.set_bid(bid).unwrap_err();
        hb.set_tricks(tricks).unwrap_err();

        hb.build().unwrap_err();
    }

    #[test]
    fn build_early() {
        let contract = emballage();

        let hb = HandBuilder::new(contract);
        hb.build().unwrap_err();
    }

    #[test]
    fn builder_edge() {
        let contract = misere();

        let mut hb = HandBuilder::new(contract);

        let c = p!(1);
        hb.set_bid(t!(1)).unwrap();

        hb.set_contractors(&c).unwrap();
        hb.set_tricks(&[t!(0)]).unwrap();
        let hand = hb.build().unwrap();

        assert!(hand.bid.is_none());

        let scores = hand.get_scores().unwrap();
        assert_eq!(scores, [-4, 12, -4, -4]);
    }

    #[test]
    fn builder_contractors_mismatch() {
        let contract = emballage();

        let mut hb = HandBuilder::new(contract);

        let c = p!(1);

        hb.set_contractors(&c).unwrap_err();
    }

    #[test]
    fn builder_contractors_mismatch_2() {
        let contract = misere();

        let mut hb = HandBuilder::new(contract);

        hb.set_tricks(&[t!(0)]).unwrap_err();

        let c = p!(0);
        hb.set_contractors(&c).unwrap();
        hb.set_tricks(&[t!(0)]).unwrap();

        let hand = hb.build().unwrap();
        assert_eq!(hand.contractors_tricks, p_and_t!(0).to_vec());

        let scores = hand.get_scores().unwrap();
        assert_eq!(scores, [12, -4, -4, -4]);
    }

    #[test]
    fn scores_unordered_with_bid() {
        let hand = Hand {
            contract: emballage(),
            contractors_tricks: vec![(PlayerId(3), t!(8)), (PlayerId(2), t!(8))],
            bid: Some(t!(9)),
        };

        let scores = hand.get_scores().unwrap();
        assert_eq!(scores, [6, 6, -6, -6]);
    }

    #[test]
    fn infer_last_score() {
        let hand = Hand {
            contract: misere(),
            contractors_tricks: vec![
                (PlayerId(3), t!(0)),
                (PlayerId(2), t!(0)),
                (PlayerId(1), t!(2)),
            ],
            bid: Some(t!(9)),
        };

        let scores = hand.get_scores().unwrap();
        assert_eq!(scores, [0, -24, 12, 12]);
    }
}
