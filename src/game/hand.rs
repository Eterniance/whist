use super::{GameError, rules::Contract};
use crate::{
    game::{
        contractors::{Contractors, ContractorsKind, ContractorsScore},
        players::{PlayerId, PlayerIdAndScore},
    },
    gamemodes::{Score, TOTAL_TRICKS},
};
use std::{collections::HashMap, rc::Rc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("The score sum cannot be zero")]
    WrongScore,
}

#[derive(Debug)]
pub struct Hand {
    pub contractors: Contractors,
    contract: Rc<Contract>,
    bid: Option<i16>,
    tricks: i16,
}

impl Hand {
    #[must_use]
    pub fn gamemode_name(&self) -> String {
        self.contract.gamemode.name()
    }

    #[must_use]
    pub fn get_score(&self) -> i16 {
        let tricks = self
            .contract
            .max_bid
            .map_or(self.tricks, |max| self.tricks.clamp(0, max));
        let adjusted_tricks = self.bid.map_or(tricks, |bid| {
            let diff = bid - self.contract.min_tricks();
            tricks - diff
        });
        self.contract.gamemode.get_score(adjusted_tricks)
    }

    #[must_use]
    pub fn get_contractors_score(&self) -> ContractorsScore {
        match &self.contractors {
            Contractors::Solo(id) => {
                let score = self.get_score();
                ContractorsScore::Solo(PlayerIdAndScore::new(id.clone(), score))
            }
            Contractors::Team(id1, id2) => {
                let score = self.get_score();
                ContractorsScore::Team(
                    PlayerIdAndScore::new(id1.clone(), score),
                    PlayerIdAndScore::new(id2.clone(), score),
                )
            }
            Contractors::Other(player_id_and_scores) => {
                ContractorsScore::Other(player_id_and_scores.clone())
            }
        }
    }
}

#[derive(Debug)]
pub enum InputRequest {
    ContractorsSolo,
    ContractorsTeam,
    ContractorsOther,
    Bid { min: i16, max: i16 },
    Done,
    Cancel,
}

#[derive(Debug)]
pub struct HandBuilder {
    contract: Rc<Contract>,
    contractors: Option<Contractors>,
    bid: Option<i16>,
    tricks: i16,
}

impl HandBuilder {
    #[must_use]
    pub const fn new(contract: Rc<Contract>) -> Self {
        Self {
            contract,
            contractors: None,
            bid: None,
            tricks: 0,
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

    fn contract_request(&self) -> InputRequest {
        match self.contract.contractors_kind {
            ContractorsKind::Solo => InputRequest::ContractorsSolo,
            ContractorsKind::Team => InputRequest::ContractorsTeam,
            ContractorsKind::Other => InputRequest::ContractorsOther,
        }
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
    pub fn set_contractors(&mut self, c: Contractors) -> Result<(), GameError> {
        if self.contract.contractors_kind != c {
            return Err(GameError::HandBuildError(
                "Contractors type does not match".to_string(),
            ));
        }

        self.contractors = Some(c);
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
    pub fn set_bid(&mut self, bid: i16) -> Result<(), GameError> {
        if let Some(max_bid) = self.contract.max_bid {
            if !(self.contract.min_tricks()..=max_bid).contains(&bid) {
                return Err(GameError::HandBuildError("Bid out of range".to_string()));
            }
            self.bid = Some(bid);
        } else {
            self.bid = None;
        }
        Ok(())
    }

    pub fn set_tricks(&mut self, tricks: i16) {
        self.tricks = tricks.clamp(0, TOTAL_TRICKS);
    }

    /// Builds the hand from the collected contract parameters.
    ///
    /// All required components must be set before building the hand.
    ///
    /// # Errors
    ///
    /// Returns an error if the contractors are missing, or if a bid is required
    /// by the contract but has not been set.
    pub fn build(self) -> Result<Hand, GameError> {
        let contractors = self
            .contractors
            .ok_or_else(|| GameError::HandBuildError("No contractors".to_string()))?;
        if self.contract.max_bid.is_some() && self.bid.is_none() {
            return Err(GameError::HandBuildError("Missing bid".to_string()));
        }
        Ok(Hand {
            contract: self.contract,
            contractors,
            bid: self.bid,
            tricks: self.tricks,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HandRecap {
    pub gamemode_name: String,
    pub scores: HashMap<PlayerId, i16>,
    pub tricks: i16,
    pub contractors: ContractorsScore,
    pub bid: Option<i16>,
}
