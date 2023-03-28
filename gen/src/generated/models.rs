use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};
pub mod applications;
pub mod common;
pub mod components;
pub mod config;
pub mod content;
pub mod dates;
pub mod destiny;
pub mod entities;
pub mod exceptions;
pub mod fireteam;
pub mod forum;
pub mod forums;
pub mod groups_v_2;
pub mod ignores;
pub mod interpolation;
pub mod links;
pub mod queries;
pub mod social;
pub mod streaming;
pub mod tags;
pub mod tokens;
pub mod trending;
pub mod user;
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
pub struct DestinyBaseItemComponentSetOfint32 {

    /// No documentation provided.
    pub objectives: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent,
    /// No documentation provided.
    pub perks: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemPerksComponent,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyBaseItemComponentSetOfint64 {

    /// No documentation provided.
    pub objectives: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent,
    /// No documentation provided.
    pub perks: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemPerksComponent,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyBaseItemComponentSetOfuint32 {

    /// No documentation provided.
    pub objectives: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent,
    /// No documentation provided.
    pub perks: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemComponentSetOfint32 {

    /// No documentation provided.
    pub instances: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent,
    /// No documentation provided.
    pub objectives: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent,
    /// No documentation provided.
    pub perks: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemPerksComponent,
    /// No documentation provided.
    pub plug_objectives: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemPlugObjectivesComponent,
    /// No documentation provided.
    pub plug_states: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent,
    /// No documentation provided.
    pub render_data: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemRenderComponent,
    /// No documentation provided.
    pub reusable_plugs: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemReusablePlugsComponent,
    /// No documentation provided.
    pub sockets: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent,
    /// No documentation provided.
    pub stats: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemStatsComponent,
    /// No documentation provided.
    pub talent_grids: crate::generated::models::DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemComponentSetOfint64 {

    /// No documentation provided.
    pub instances: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemInstanceComponent,
    /// No documentation provided.
    pub objectives: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent,
    /// No documentation provided.
    pub perks: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemPerksComponent,
    /// No documentation provided.
    pub plug_objectives: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemPlugObjectivesComponent,
    /// No documentation provided.
    pub plug_states: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent,
    /// No documentation provided.
    pub render_data: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemRenderComponent,
    /// No documentation provided.
    pub reusable_plugs: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemReusablePlugsComponent,
    /// No documentation provided.
    pub sockets: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent,
    /// No documentation provided.
    pub stats: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemStatsComponent,
    /// No documentation provided.
    pub talent_grids: crate::generated::models::DictionaryComponentResponseOfint64AndDestinyItemTalentGridComponent,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemComponentSetOfuint32 {

    /// No documentation provided.
    pub instances: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent,
    /// No documentation provided.
    pub objectives: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent,
    /// No documentation provided.
    pub perks: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent,
    /// No documentation provided.
    pub plug_objectives: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent,
    /// No documentation provided.
    pub plug_states: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent,
    /// No documentation provided.
    pub render_data: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent,
    /// No documentation provided.
    pub reusable_plugs: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent,
    /// No documentation provided.
    pub sockets: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent,
    /// No documentation provided.
    pub stats: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent,
    /// No documentation provided.
    pub talent_grids: crate::generated::models::DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorSaleItemSetComponentOfDestinyPublicVendorSaleItemComponent {

    /// No documentation provided.
    pub sale_items: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorSaleItemSetComponentOfDestinyVendorSaleItemComponent {

    /// No documentation provided.
    pub sale_items: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemPerksComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemPlugObjectivesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemRenderComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemReusablePlugsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemStatsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint32AndDestinyVendorSaleItemComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterProgressionComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterRecordsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterRenderComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCollectiblesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCraftablesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyCurrenciesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyInventoryComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemInstanceComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemPerksComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemPlugObjectivesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemRenderComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemReusablePlugsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemStatsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemTalentGridComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyKiosksComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyLoadoutsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyPlugSetsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyPresentationNodesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyStringVariablesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyPublicVendorComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyVendorCategoriesComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndDestinyVendorComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndPersonalDestinyVendorSaleItemSetComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfuint32AndPublicDestinyVendorSaleItemSetComponent {

    /// No documentation provided.
    pub data: i32,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalAlert {

    /// No documentation provided.
    pub alert_html: String,
    /// No documentation provided.
    pub alert_key: String,
    /// No documentation provided.
    pub alert_level: crate::generated::models::GlobalAlertLevel,
    /// No documentation provided.
    pub alert_link: String,
    /// No documentation provided.
    pub alert_timestamp: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub alert_type: crate::generated::models::GlobalAlertType,
    /// No documentation provided.
    pub stream_info: crate::generated::models::StreamInfo,
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
pub struct SearchResultOfContentItemPublicContract {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::content::ContentItemPublicContract>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfDestinyEntitySearchResultItem {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::destiny::definitions::DestinyEntitySearchResultItem>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfFireteamResponse {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::fireteam::FireteamResponse>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfFireteamSummary {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::fireteam::FireteamSummary>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupBan {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupBan>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupMember {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupMember>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupMemberApplication {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupMemberApplication>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupMembership {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupMembership>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupPotentialMembership {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupPotentialMembership>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfGroupV2Card {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupV2Card>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfPostResponse {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::forum::PostResponse>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultOfTrendingEntry {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::trending::TrendingEntry>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterActivitiesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::characters::DestinyCharacterActivitiesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::characters::DestinyCharacterComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterProgressionComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::characters::DestinyCharacterProgressionComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterRecordsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::records::DestinyCharacterRecordsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterRenderComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::characters::DestinyCharacterRenderComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCollectiblesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::collectibles::DestinyCollectiblesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyCurrenciesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::inventory::DestinyCurrenciesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyInventoryComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::inventory::DestinyInventoryComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::items::DestinyItemComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemInstanceComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::items::DestinyItemInstanceComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemObjectivesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::items::DestinyItemObjectivesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemPerksComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::items::DestinyItemPerksComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemPlugObjectivesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::items::DestinyItemPlugObjectivesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemRenderComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::items::DestinyItemRenderComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemReusablePlugsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::items::DestinyItemReusablePlugsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemSocketsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::items::DestinyItemSocketsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemStatsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::items::DestinyItemStatsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemTalentGridComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::items::DestinyItemTalentGridComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyKiosksComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::kiosks::DestinyKiosksComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyLoadoutsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::loadouts::DestinyLoadoutsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyMetricsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::metrics::DestinyMetricsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyPlatformSilverComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::inventory::DestinyPlatformSilverComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyPlugSetsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::plug_sets::DestinyPlugSetsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyPresentationNodesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::presentation::DestinyPresentationNodesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileCollectiblesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::collectibles::DestinyProfileCollectiblesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::profiles::DestinyProfileComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileProgressionComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::profiles::DestinyProfileProgressionComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileRecordsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::records::DestinyProfileRecordsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyProfileTransitoryComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::profiles::DestinyProfileTransitoryComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinySocialCommendationsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::social::DestinySocialCommendationsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyStringVariablesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::string_variables::DestinyStringVariablesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyVendorCategoriesComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::vendors::DestinyVendorCategoriesComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyVendorComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::vendors::DestinyVendorComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyVendorGroupComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::components::vendors::DestinyVendorGroupComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleComponentResponseOfDestinyVendorReceiptsComponent {

    /// No documentation provided.
    pub data: crate::generated::models::destiny::entities::profiles::DestinyVendorReceiptsComponent,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamInfo {

    /// No documentation provided.
    pub channel_name: String,
}
