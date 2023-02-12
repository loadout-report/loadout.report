

use serde::{Serialize, Deserialize};


/// Represents the status and other related information for a challenge that is - or was - available to a player. 
/// A challenge is a bonus objective, generally tacked onto Quests or Activities, that provide additional variations on play.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyChallengeStatus {

    /// The progress - including completion status - of the active challenge.
    pub objective: i32,
}
