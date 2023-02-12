pub mod forums;
pub mod forum;
pub mod groups_v_2;
pub mod links;
pub mod ignores;
pub mod entities;
pub mod destiny;
pub mod interpolation;
pub mod queries;
pub mod content;
pub mod exceptions;
pub mod fireteam;
pub mod config;
pub mod user;
pub mod tokens;
pub mod trending;
pub mod components;
pub mod dates;
pub mod streaming;
pub mod applications;

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyPresentationNodesComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyMetricsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemStatsComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileProgressionComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemObjectivesComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemPlugObjectivesComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyInventoryComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyPlugSetsComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfPostResponse {

    /// No documentation provided.
    pub total_results: i32,
    /// No documentation provided.
    pub query: i32,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub results: i32,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyBaseItemComponentSetOfuint32 {

    /// No documentation provided.
    pub objectives: i32,
    /// No documentation provided.
    pub perks: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemComponentSetOfint64 {

    /// No documentation provided.
    pub reusable_plugs: i32,
    /// No documentation provided.
    pub instances: i32,
    /// No documentation provided.
    pub plug_objectives: i32,
    /// No documentation provided.
    pub stats: i32,
    /// No documentation provided.
    pub plug_states: i32,
    /// No documentation provided.
    pub talent_grids: i32,
    /// No documentation provided.
    pub render_data: i32,
    /// No documentation provided.
    pub perks: i32,
    /// No documentation provided.
    pub objectives: i32,
    /// No documentation provided.
    pub sockets: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupBan {

    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub total_results: i32,
    /// No documentation provided.
    pub query: i32,
    /// No documentation provided.
    pub results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub has_more: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyPlugSetsComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyInventoryComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterProgressionComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCurrenciesComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorSaleItemSetComponentOfDestinyVendorSaleItemComponent {

    /// No documentation provided.
    pub sale_items: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemPerksComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCraftablesComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemPerksComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyVendorComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemTalentGridComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemPlugObjectivesComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupMember {

    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub query: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub results: i32,
    /// No documentation provided.
    pub total_results: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemComponentSetOfint32 {

    /// No documentation provided.
    pub render_data: i32,
    /// No documentation provided.
    pub objectives: i32,
    /// No documentation provided.
    pub plug_states: i32,
    /// No documentation provided.
    pub reusable_plugs: i32,
    /// No documentation provided.
    pub stats: i32,
    /// No documentation provided.
    pub sockets: i32,
    /// No documentation provided.
    pub instances: i32,
    /// No documentation provided.
    pub talent_grids: i32,
    /// No documentation provided.
    pub perks: i32,
    /// No documentation provided.
    pub plug_objectives: i32,
}

/// The types of credentials the Accounts system supports. This is the external facing enum used in place of the internal-only Bungie.SharedDefinitions.CredentialType.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BungieCredentialType {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Xuid = 1,
    /// No documentation provided.
    Psnid = 2,
    /// No documentation provided.
    Wlid = 3,
    /// No documentation provided.
    Fake = 4,
    /// No documentation provided.
    Facebook = 5,
    /// No documentation provided.
    Google = 8,
    /// No documentation provided.
    Windows = 9,
    /// No documentation provided.
    DemonId = 10,
    /// No documentation provided.
    SteamId = 12,
    /// No documentation provided.
    BattleNetId = 14,
    /// No documentation provided.
    StadiaId = 16,
    /// No documentation provided.
    TwitchId = 18,
    /// No documentation provided.
    EgsId = 20,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterRecordsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalAlert {

    /// No documentation provided.
    pub alert_link: String,
    /// No documentation provided.
    pub alert_type: i32,
    /// No documentation provided.
    pub alert_key: String,
    /// No documentation provided.
    pub stream_info: i32,
    /// No documentation provided.
    pub alert_timestamp: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub alert_level: i32,
    /// No documentation provided.
    pub alert_html: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemPerksComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GlobalAlertType {
    /// No documentation provided.
    GlobalAlert = 0,
    /// No documentation provided.
    StreamingAlert = 1,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupV2Card {

    /// No documentation provided.
    pub total_results: i32,
    /// No documentation provided.
    pub query: i32,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub has_more: bool,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub results: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterActivitiesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyKiosksComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemRenderComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyStringVariablesComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemSocketsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemRenderComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCurrenciesComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupMemberApplication {

    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub results: i32,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyPresentationNodesComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupPotentialMembership {

    /// No documentation provided.
    pub query: i32,
    /// No documentation provided.
    pub results: i32,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub replacement_continuation_token: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupMembership {

    /// No documentation provided.
    pub total_results: i32,
    /// No documentation provided.
    pub results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: i32,
    /// No documentation provided.
    pub replacement_continuation_token: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyVendorSaleItemComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyBaseItemComponentSetOfint64 {

    /// No documentation provided.
    pub objectives: i32,
    /// No documentation provided.
    pub perks: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfFireteamResponse {

    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub total_results: i32,
    /// No documentation provided.
    pub query: i32,
    /// No documentation provided.
    pub results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub has_more: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterRecordsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemStatsComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyPlatformSilverComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemPlugObjectivesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileCollectiblesComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyKiosksComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
}

/// The types of membership the Accounts system supports. This is the external facing enum used in place of the internal-only Bungie.SharedDefinitions.MembershipType.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum BungieMembershipType {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    TigerXbox = 1,
    /// No documentation provided.
    TigerPsn = 2,
    /// No documentation provided.
    TigerSteam = 3,
    /// No documentation provided.
    TigerBlizzard = 4,
    /// No documentation provided.
    TigerStadia = 5,
    /// No documentation provided.
    TigerEgs = 6,
    /// No documentation provided.
    TigerDemon = 10,
    /// No documentation provided.
    BungieNext = 254,
    /// "All" is only valid for searching capabilities: you need to pass the actual matching BungieMembershipType for any query where you pass a known membershipId.
    All = -1,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfContentItemPublicContract {

    /// No documentation provided.
    pub results: i32,
    /// No documentation provided.
    pub query: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub total_results: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyVendorCategoriesComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileTransitoryComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterProgressionComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyStringVariablesComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemReusablePlugsComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemStatsComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyPublicVendorComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyVendorGroupComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfDestinyEntitySearchResultItem {

    /// No documentation provided.
    pub query: i32,
    /// No documentation provided.
    pub results: i32,
    /// No documentation provided.
    pub total_results: i32,
    /// No documentation provided.
    pub has_more: bool,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub replacement_continuation_token: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterRenderComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemComponentSetOfuint32 {

    /// No documentation provided.
    pub render_data: i32,
    /// No documentation provided.
    pub plug_objectives: i32,
    /// No documentation provided.
    pub talent_grids: i32,
    /// No documentation provided.
    pub perks: i32,
    /// No documentation provided.
    pub reusable_plugs: i32,
    /// No documentation provided.
    pub plug_states: i32,
    /// No documentation provided.
    pub instances: i32,
    /// No documentation provided.
    pub objectives: i32,
    /// No documentation provided.
    pub stats: i32,
    /// No documentation provided.
    pub sockets: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfFireteamSummary {

    /// No documentation provided.
    pub query: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub total_results: i32,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub results: i32,
    /// No documentation provided.
    pub replacement_continuation_token: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemInstanceComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndPersonalDestinyVendorSaleItemSetComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCollectiblesComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorSaleItemSetComponentOfDestinyPublicVendorSaleItemComponent {

    /// No documentation provided.
    pub sale_items: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemRenderComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyVendorReceiptsComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemReusablePlugsComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemReusablePlugsComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfTrendingEntry {

    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub results: i32,
    /// No documentation provided.
    pub query: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileRecordsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyVendorComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCollectiblesComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyBaseItemComponentSetOfint32 {

    /// No documentation provided.
    pub objectives: i32,
    /// No documentation provided.
    pub perks: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemTalentGridComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyVendorCategoriesComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterRenderComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamInfo {

    /// No documentation provided.
    pub channel_name: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileComponent {

    /// No documentation provided.
    pub privacy: i32,
    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GlobalAlertLevel {
    /// No documentation provided.
    Unknown = 0,
    /// No documentation provided.
    Blue = 1,
    /// No documentation provided.
    Yellow = 2,
    /// No documentation provided.
    Red = 3,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndPublicDestinyVendorSaleItemSetComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemInstanceComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent {

    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemComponent {

    /// No documentation provided.
    pub data: i32,
    /// No documentation provided.
    pub privacy: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}
