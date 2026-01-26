use crate::{CollectedTricks, TOTAL_PLAYERS, Tricks, contracts::Contract, players::PlayerId};
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
    pub contractors: Box<[PlayerId]>,
    contract: Contract,
    bid: Option<Tricks>,
    tricks: Tricks,
}

impl Hand {
    #[must_use]
    pub const fn gamemode_name(&self) -> &'static str {
        self.contract.name
    }

    /// The tricks number adjusted with the bid.
    ///
    /// Clamp the tricks number to the maximum allowed tricks if any
    /// and subtract the bid delta from the default minimum tricks.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_effective_tricks(&self) -> Tricks {
        let tricks = self.contract.max_bid.map_or(self.tricks, |max| {
            let inner = self.tricks.get().clamp(0, max.get());
            Tricks::new(inner).expect("inner has been clamp between Tricks range")
        });

        self.bid.map_or(tricks, |bid| {
            let diff = bid
                .checked_sub(self.contract.min_tricks())
                .expect("Bid should be greater than min_tricks");
            tricks.saturating_sub(diff)
        })
    }

    #[must_use]
    pub fn get_score(&self) -> i16 {
        let adjusted_tricks = self.get_effective_tricks();
        let tricks = CollectedTricks::new(self.tricks, adjusted_tricks);
        self.contract.gamemode.calculate_score(tricks)
    }

    #[must_use]
    pub fn as_recap(self, scores: [i16; TOTAL_PLAYERS]) -> HandRecap {
        HandRecap {
            scores,
            gamemode_name: self.gamemode_name().to_string(),
            tricks: self.tricks,
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
    contractors: Option<Box<[PlayerId]>>,
    bid: Option<Tricks>,
    tricks: Option<Tricks>,
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
        if &u8::try_from(c.len()).expect("Only 4 players max")
            > self.contract.contractors_kind.end()
        {
            return Err(HandBuildError("Contractors type does not match"));
        }

        self.contractors = Some(c.into());
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

    pub const fn set_tricks(&mut self, tricks: Tricks) {
        self.tricks = Some(tricks);
    }

    /// Builds the hand from the collected contract parameters.
    ///
    /// All required components must be set before building the hand.
    ///
    /// # Errors
    ///
    /// Returns an error if the contractors are missing, or if a bid is required
    /// by the contract but has not been set.
    pub fn build(self) -> Result<Hand, HandBuildError> {
        let contractors = self.contractors.ok_or(HandBuildError("No contractors"))?;
        let tricks = self.tricks.ok_or(HandBuildError("No tricks set"))?;
        if self.contract.max_bid.is_some() && self.bid.is_none() {
            return Err(HandBuildError("Missing bid"));
        }
        Ok(Hand {
            contract: self.contract,
            contractors,
            bid: self.bid,
            tricks,
        })
    }
}

#[derive(Debug, Error, Clone, Eq, PartialEq)]
#[error("{0}")]
pub struct HandBuildError(&'static str);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HandRecap {
    pub scores: [i16; 4],
    pub gamemode_name: String,
    pub tricks: Tricks,
    pub bid: Option<Tricks>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        gamemodes::TricksChase,
        p,
        scoring::{CappedChase, ExactTricks},
        t,
    };

    fn emballage() -> Contract {
        let rules = TricksChase::new(t!(8), 2, 1);
        Contract {
            name: "Emballage",
            max_bid: Some(Tricks::MAX_TRICKS),
            gamemode: Box::new(rules),
            contractors_kind: 2..=2,
        }
    }

    fn misere() -> Contract {
        let rules = ExactTricks::new(12);
        Contract {
            name: "Misere",
            max_bid: None,
            gamemode: Box::new(rules),
            contractors_kind: 1..=3,
        }
    }

    fn seul() -> Contract {
        let rules = CappedChase::new(t!(6), 6, 3, t!(8));
        Contract {
            name: "Seul",
            max_bid: Some(t!(8)),
            gamemode: Box::new(rules),
            contractors_kind: 1..=1,
        }
    }

    #[test]
    fn hand_builder_ok() {
        let contract = emballage();

        let contractors = p!(1, 2);
        let bid = t!(9);
        let tricks = t!(10);

        let mut hb = HandBuilder::new(contract);

        hb.set_contractors(&contractors).unwrap();
        hb.set_bid(bid).unwrap();
        hb.set_tricks(tricks);

        let hand = hb.build().unwrap();

        assert_eq!(*hand.contractors, contractors);
        assert_eq!(hand.bid, Some(bid));

        assert_eq!(hand.tricks, tricks);
    }

    #[test]
    fn hand_builder_fails() {
        let contract = emballage();

        let contractors = p!(1, 2, 3);
        let bid = t!(7);
        let tricks = t!(10);

        let mut hb = HandBuilder::new(contract);

        hb.set_contractors(&contractors).unwrap_err();

        hb.set_bid(bid).unwrap_err();
        hb.set_tricks(tricks);

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
        hb.set_tricks(t!(0));
        let hand = hb.build().unwrap();

        assert!(hand.bid.is_none());
        assert_eq!(hand.get_score(), 12);
    }

    #[test]
    fn adjusted_tricks_without_bid() {
        let contract = misere();

        let contractors = p!(1);
        let tricks = t!(5);

        let mut hb = HandBuilder::new(contract);
        hb.set_contractors(&contractors).unwrap();
        hb.set_tricks(tricks);

        let hand = hb.build().unwrap();

        assert_eq!(hand.get_effective_tricks(), tricks);
    }

    #[test]
    fn adjusted_tricks_with_bid() {
        let contract = emballage();

        let contractors = p!(1, 2);
        let bid = t!(10);
        let tricks = t!(10);

        let mut hb = HandBuilder::new(contract);
        hb.set_contractors(&contractors).unwrap();
        hb.set_bid(bid).unwrap();
        hb.set_tricks(tricks);

        let hand = hb.build().unwrap();

        assert_eq!(hand.get_effective_tricks(), t!(8));
    }

    #[test]
    fn adjusted_tricks_with_bid_saturating() {
        let contract = emballage();

        let contractors = p!(1, 2);
        let bid = Tricks::MAX_TRICKS;
        let tricks = t!(3);

        let mut hb = HandBuilder::new(contract);
        hb.set_contractors(&contractors).unwrap();
        hb.set_bid(bid).unwrap();
        hb.set_tricks(tricks);

        let hand = hb.build().unwrap();

        assert_eq!(hand.get_effective_tricks(), t!(0));
    }

    #[test]
    fn adjusted_tricks_clamp() {
        let contract = seul();

        let contractors = p!(1);

        let bid = t!(7);
        let tricks = t!(12);

        let mut hb = HandBuilder::new(contract);
        hb.set_contractors(&contractors).unwrap();
        hb.set_bid(bid).unwrap();
        hb.set_tricks(tricks);

        let hand = hb.build().unwrap();

        let min = hand.contract.min_tricks();
        let diff = bid.checked_sub(min).unwrap();
        let expected = t!(8).saturating_sub(diff);

        assert_eq!(hand.get_effective_tricks(), expected);
    }
}
