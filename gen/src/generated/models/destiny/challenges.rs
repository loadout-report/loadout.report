use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Represents the status and other related information for a challenge that is - or was - available to a player. 
/// A challenge is a bonus objective, generally tacked onto Quests or Activities, that provide additional variations on play.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyChallengeStatus {

    /// The progress - including completion status - of the active challenge.
    pub objective: crate::generated::models::destiny::quests::DestinyObjectiveProgress,
}
