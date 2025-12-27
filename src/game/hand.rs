use super::{
    GameError,
    players::Contractors,
    rules::{Contract, ContractorsKind},
};
use crate::{game::players::Players, gamemodes::Score};
use async_trait::async_trait;
use std::{ops::RangeInclusive, sync::Arc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Requester {
    async fn ask_bid(&self, range: RangeInclusive<i16>) -> Result<i16, InputError>;
    async fn ask_name(&self) -> Result<String, InputError>;
    async fn pick_names(
        &self,
        names_list: Vec<String>,
        names_number: usize,
    ) -> Result<Vec<String>, InputError>;

    async fn pick_name(&self, names_list: Vec<String>) -> Option<String> {
        self.pick_names(names_list, 1).await.ok()?.first().cloned()
    }
}

#[derive(Debug)]
pub struct Hand {
    pub contractors: Contractors,
    contract: Arc<Contract>,
    bid: Option<i16>,
}

impl Score for Hand {
    fn min_tricks(&self) -> i16 {
        self.contract.min_tricks()
    }

    fn calculate_score(&self, tricks: i16) -> (i16, crate::gamemodes::GameResult) {
        let tricks = self
            .contract
            .max_bid
            .map_or(tricks, |max| tricks.clamp(0, max));
        let adjusted_tricks = self.bid.map_or(tricks, |bid| {
            let diff = bid - self.min_tricks();
            tricks - diff
        });
        self.contract.gamemode.calculate_score(adjusted_tricks)
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

pub struct HandBuilder {
    contract: Arc<Contract>,
    contractors: Option<Contractors>,
    bid: Option<i16>,
}

impl HandBuilder {
    #[must_use]
    pub const fn new(contract: Arc<Contract>) -> Self {
        Self {
            contract,
            contractors: None,
            bid: None,
        }
    }

    #[must_use]
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
        }
        InputRequest::Done
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
        })
    }
}

/// Interactively builds a `Hand` by requesting missing inputs from a requester.
///
/// The function drives a `HandBuilder` until all required information
/// (contractors, bid) is provided, using the given `Requester` to obtain user
/// input asynchronously.
///
/// # Errors
///
/// Returns an error if an invalid contractors selection or bid is provided,
/// or if the requester fails when selecting multiple names.
///
/// # Panics
///
/// Panics if a selected player name cannot be resolved to a player ID,
/// which assumes that `players` has been properly initialized and kept
/// consistent with the requester.
pub async fn build_hand<R: Requester + Sync + Send>(
    contract: Arc<Contract>,
    players: Arc<Players>,
    requester: Arc<R>,
) -> Result<Hand, GameError> {
    let mut b = HandBuilder::new(contract);

    loop {
        match b.next_request() {
            InputRequest::ContractorsSolo => {
                let list = players.names();
                if let Some(name) = requester.pick_name(list).await {
                    let id = players.get_id(&name).expect("Players is initialized");
                    b.set_contractors(Contractors::Solo(id))?;
                }
            }
            InputRequest::ContractorsTeam => {
                let list = players.names();
                let names = requester.pick_names(list, 2).await?;
                let id1 = players.get_id(&names[0]).expect("Players is initialized");
                let id2 = players.get_id(&names[1]).expect("Players is initialized");
                b.set_contractors(Contractors::Team(id1, id2))?;
            }
            InputRequest::ContractorsOther => todo!(),
            InputRequest::Bid { min, max } => {
                let bid = requester.ask_bid(min..=max).await?;
                b.set_bid(bid)?;
            }
            InputRequest::Done => {
                let hand = b.build()?;
                return Ok(hand);
            }
            InputRequest::Cancel => todo!(),
        }
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
        let scorables: Vec<_> = select_rules(&GameRules::Dutch);
        let emballage = Arc::new(scorables[0].clone());

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
            other => panic!("Expected Bid request, got {other:?}"),
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
        let scorables = select_rules(&GameRules::French);
        let picolo = Arc::new(scorables[2].clone());

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
        let scorables = select_rules(&GameRules::Dutch);
        let emballage = Arc::new(scorables[0].clone());

        let builder = HandBuilder::new(Arc::clone(&emballage));
        let err = builder.build().unwrap_err();
        assert!(matches!(err, GameError::HandBuildError(_)));

        let mut builder = HandBuilder::new(Arc::clone(&emballage));

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
