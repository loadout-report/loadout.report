use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiUsage {

    /// Counts for on API calls made for the time range.
    pub api_calls: Vec<crate::generated::models::applications::Series>,
    /// Instances of blocked requests or requests that crossed the warn threshold during the time range.
    pub throttled_requests: Vec<crate::generated::models::applications::Series>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {

    /// Unique ID assigned to the application
    pub application_id: i32,
    /// Date the application was first added to our database.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// Date the first time the application status entered the 'Public' status.
    pub first_published: chrono::DateTime<chrono::Utc>,
    /// Link to website for the application where a user can learn more about the app.
    pub link: String,
    /// Name of the application
    pub name: String,
    /// Value of the Origin header sent in requests generated by this application.
    pub origin: String,
    /// An optional override for the Authorize view name.
    pub override_authorize_view_name: String,
    /// URL used to pass the user's authorization code to the application
    pub redirect_url: String,
    /// Permissions the application needs to work
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub scope: i64,
    /// Current status of the application.
    pub status: crate::generated::models::applications::ApplicationStatus,
    /// Date the application status last changed.
    pub status_changed: chrono::DateTime<chrono::Utc>,
    /// List of team members who manage this application on Bungie.net. Will always consist of at least the application owner.
    pub team: Vec<crate::generated::models::applications::ApplicationDeveloper>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDeveloper {

    /// No documentation provided.
    pub api_eula_version: i32,
    /// No documentation provided.
    pub role: crate::generated::models::applications::DeveloperRole,
    /// No documentation provided.
    pub user: crate::generated::models::user::UserInfoCard,
}

/// No documentation provided.
/// todo: bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum ApplicationScopes {
    /// Read basic user profile information such as the user's handle, avatar icon, etc.
    ReadBasicUserProfile = 1,
    /// Read Group/Clan Forums, Wall, and Members for groups and clans that the user has joined.
    ReadGroups = 2,
    /// Write Group/Clan Forums, Wall, and Members for groups and clans that the user has joined.
    WriteGroups = 4,
    /// Administer Group/Clan Forums, Wall, and Members for groups and clans that the user is a founder or an administrator.
    AdminGroups = 8,
    /// Create new groups, clans, and forum posts, along with other actions that are reserved for Bungie.net elevated scope: not meant to be used by third party applications.
    BnetWrite = 16,
    /// Move or equip Destiny items
    MoveEquipDestinyItems = 32,
    /// Read Destiny 1 Inventory and Vault contents. For Destiny 2, this scope is needed to read anything regarded as private. This is the only scope a Destiny 2 app needs for read operations against Destiny 2 data such as inventory, vault, currency, vendors, milestones, progression, etc.
    ReadDestinyInventoryAndVault = 64,
    /// Read user data such as who they are web notifications, clan/group memberships, recent activity, muted users.
    ReadUserData = 128,
    /// Edit user data such as preferred language, status, motto, avatar selection and theme.
    EditUserData = 256,
    /// Access vendor and advisor data specific to a user. OBSOLETE. This scope is only used on the Destiny 1 API.
    ReadDestinyVendorsAndAdvisors = 512,
    /// Read offer history and claim and apply tokens for the user.
    ReadAndApplyTokens = 1024,
    /// Can perform actions that will result in a prompt to the user via the Destiny app.
    AdvancedWriteActions = 2048,
    /// Can use the partner offer api to claim rewards defined for a partner
    PartnerOfferGrant = 4096,
    /// Allows an app to query sensitive information like unlock flags and values not available through normal methods.
    DestinyUnlockValueQuery = 8192,
    /// Allows an app to query sensitive user PII, most notably email information.
    UserPiiRead = 16384,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ApplicationStatus {
    /// No value assigned
    None = 0,
    /// Application exists and works but will not appear in any public catalog. New applications start in this state, test applications will remain in this state.
    Private = 1,
    /// Active applications that can appear in an catalog.
    Public = 2,
    /// Application disabled by the owner. All authorizations will be treated as terminated while in this state. Owner can move back to private or public state.
    Disabled = 3,
    /// Application has been blocked by Bungie. It cannot be transitioned out of this state by the owner. Authorizations are terminated when an application is in this state.
    Blocked = 4,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Datapoint {

    /// Count associated with timestamp
    pub count: Option<f64>,
    /// Timestamp for the related count.
    pub time: chrono::DateTime<chrono::Utc>,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DeveloperRole {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Owner = 1,
    /// No documentation provided.
    TeamMember = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {

    /// Collection of samples with time and value.
    pub datapoints: Vec<crate::generated::models::applications::Datapoint>,
    /// Target to which to datapoints apply.
    pub target: String,
}
