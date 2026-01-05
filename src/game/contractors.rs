use crate::game::players::{PlayerId, PlayerIdAndScore};
use std::collections::HashMap;
use std::hash::Hash;

fn multiset_eq<T: Eq + Hash>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut counts = HashMap::new();
    for x in a {
        *counts.entry(x).or_insert(0) += 1;
    }

    for y in b {
        match counts.get_mut(y) {
            Some(c) if *c > 0 => *c -= 1,
            _ => return false,
        }
    }

    true
}

#[derive(Debug, Clone)]
#[repr(usize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContractorsScore {
    Solo(PlayerIdAndScore) = 1,
    Team(PlayerIdAndScore, PlayerIdAndScore) = 2,
    Other(Vec<PlayerIdAndScore>) = 3,
}

impl PartialEq<Contractors> for ContractorsScore {
    fn eq(&self, other: &Contractors) -> bool {
        match (self, other) {
            (Self::Solo(player_id_and_scores), Contractors::Solo(player_id)) => {
                player_id_and_scores == player_id
            }
            (Self::Team(pias1, pias2), Contractors::Team(player_id1, player_id2)) => {
                (pias1 == player_id1 && pias2 == player_id2)
                    || (pias2 == player_id1 && pias1 == player_id2)
            }
            (Self::Other(v1), Contractors::Other(v2)) => multiset_eq(v1, v2),
            _ => false,
        }
    }
}

impl PartialEq<ContractorsKind> for ContractorsScore {
    fn eq(&self, other: &ContractorsKind) -> bool {
        matches!(
            (self, other),
            (Self::Solo(_), ContractorsKind::Solo)
                | (Self::Team(_, _), ContractorsKind::Team)
                | (Self::Other(_), ContractorsKind::Other)
        )
    }
}

#[derive(Debug, Clone)]
#[repr(usize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Contractors {
    Solo(PlayerId) = 1,
    Team(PlayerId, PlayerId) = 2,
    Other(Vec<PlayerIdAndScore>) = 3,
}

impl PartialEq<ContractorsKind> for Contractors {
    fn eq(&self, other: &ContractorsKind) -> bool {
        other == self
    }
}

impl PartialEq<ContractorsScore> for Contractors {
    fn eq(&self, other: &ContractorsScore) -> bool {
        other == self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContractorsKind {
    Solo = 1,
    Team = 2,
    Other = 3,
}

impl PartialEq<Contractors> for ContractorsKind {
    fn eq(&self, other: &Contractors) -> bool {
        matches!(
            (self, other),
            (Self::Solo, Contractors::Solo(_))
                | (Self::Team, Contractors::Team(_, _))
                | (Self::Other, Contractors::Other(_))
        )
    }
}

impl PartialEq<ContractorsScore> for ContractorsKind {
    fn eq(&self, other: &ContractorsScore) -> bool {
        matches!(
            (self, other),
            (Self::Solo, ContractorsScore::Solo(_))
                | (Self::Team, ContractorsScore::Team(_, _))
                | (Self::Other, ContractorsScore::Other(_))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test_solo() {
        let c1 = ContractorsScore::Solo(PlayerIdAndScore::from_id(PlayerId(0)));
        let c2 = Contractors::Solo(PlayerId(0));
        assert_eq!(c1, ContractorsKind::Solo);
        assert_eq!(c1, c2);
        assert_eq!(c2, c1);
        assert_eq!(c2, ContractorsKind::Solo);
        assert_eq!(ContractorsKind::Solo, c1);
        assert_eq!(ContractorsKind::Solo, c2);
    }

    #[test]
    fn equality_test_team() {
        let c1 = ContractorsScore::Team(
            PlayerIdAndScore::from_id(PlayerId(0)),
            PlayerIdAndScore::from_id(PlayerId(1)),
        );
        let c2 = Contractors::Team(PlayerId(1), PlayerId(0));
        assert_eq!(c1, ContractorsKind::Team);
        assert_eq!(c1, c2);
        assert_eq!(c2, c1);
        assert_eq!(c2, ContractorsKind::Team);
        assert_eq!(ContractorsKind::Team, c1);
        assert_eq!(ContractorsKind::Team, c2);
    }

    #[test]
    fn equality_test_other_order() {
        let a0 = PlayerIdAndScore::from_id(PlayerId(0));
        let a1 = PlayerIdAndScore::from_id(PlayerId(1));
        let a2 = PlayerIdAndScore::from_id(PlayerId(2));

        let c1 = ContractorsScore::Other(vec![a0.clone(), a1.clone(), a2.clone()]);
        let c2 = Contractors::Other(vec![a2, a0, a1]);

        assert_eq!(c1, ContractorsKind::Other);
        assert_eq!(c2, ContractorsKind::Other);

        assert_eq!(c1, c2);
        assert_eq!(c2, c1);

        assert_eq!(ContractorsKind::Other, c1);
        assert_eq!(ContractorsKind::Other, c2);
    }

    #[test]
    fn equality_test_other_duplicates_match() {
        let a0 = PlayerIdAndScore::from_id(PlayerId(0));
        let a1 = PlayerIdAndScore::from_id(PlayerId(1));

        // same multiset: {a0 x2, a1 x1}
        let c1 = ContractorsScore::Other(vec![a0.clone(), a1.clone(), a0.clone()]);
        let c2 = Contractors::Other(vec![a0.clone(), a0, a1]);

        assert_eq!(c1, c2);
        assert_eq!(c2, c1);
    }

    #[test]
    fn equality_test_other_duplicates_mismatch() {
        let a0 = PlayerIdAndScore::from_id(PlayerId(0));
        let a1 = PlayerIdAndScore::from_id(PlayerId(1));

        // different multiplicities:
        // c1 has a0 x2, a1 x1
        // c2 has a0 x1, a1 x2
        let c1 = ContractorsScore::Other(vec![a0.clone(), a0.clone(), a1.clone()]);
        let c2 = Contractors::Other(vec![a0, a1.clone(), a1]);

        assert_ne!(c1, c2);
        assert_ne!(c2, c1);
    }

    #[test]
    fn equality_test_other_different_elements() {
        let a0 = PlayerIdAndScore::from_id(PlayerId(0));
        let a1 = PlayerIdAndScore::from_id(PlayerId(1));
        let a2 = PlayerIdAndScore::from_id(PlayerId(2));

        let c1 = ContractorsScore::Other(vec![a0.clone(), a1]);
        let c2 = Contractors::Other(vec![a0, a2]);

        assert_ne!(c1, c2);
        assert_ne!(c2, c1);
    }

    #[test]
    fn equality_test_other_len_mismatch() {
        let a0 = PlayerIdAndScore::from_id(PlayerId(0));
        let a1 = PlayerIdAndScore::from_id(PlayerId(1));

        let c1 = ContractorsScore::Other(vec![a0.clone(), a1]);
        let c2 = Contractors::Other(vec![a0]);

        assert_ne!(c1, c2);
        assert_ne!(c2, c1);
    }

    #[test]
    fn equality_test_cross_kind_mismatch() {
        let solo_score = ContractorsScore::Solo(PlayerIdAndScore::from_id(PlayerId(0)));
        let team = Contractors::Team(PlayerId(0), PlayerId(1));
        let other_score = ContractorsScore::Other(vec![
            PlayerIdAndScore::from_id(PlayerId(0)),
            PlayerIdAndScore::from_id(PlayerId(1)),
        ]);

        assert_ne!(solo_score, ContractorsKind::Team);
        assert_ne!(other_score, ContractorsKind::Solo);

        assert_ne!(solo_score, team);
        assert_ne!(team, other_score);
    }
}
