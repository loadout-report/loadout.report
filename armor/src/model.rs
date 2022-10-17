use std::collections::HashMap;
use gloo_worker::HandlerId;
use data::api::manifest::model::Hash;
use serde::{Deserialize, Serialize};

pub enum Msg<T> {
    Respond { output: T, id: HandlerId },
    Ready(rexie::Rexie),
}

#[derive(Copy, Clone)]
pub(crate) struct StrippedInventoryArmor {
    pub id: i32,
    pub item_instance_id: Hash,
    pub masterworked: bool,
    pub may_be_bugged: bool, // if there was an error in the parsing
    pub mobility: u8,
    pub resilience: u8,
    pub recovery: u8,
    pub discipline: u8,
    pub intellect: u8,
    pub strength: u8,

    pub energy_level: u8,
    pub energy_affinity: DestinyEnergyType,
    pub hash: Hash,
    pub slot: ArmorSlot,
    pub clazz: CharacterClass,
    pub perk: ArmorPerkOrSlot,
    pub is_exotic: bool,
    pub rarity: TierType,
    pub exotic_perk_hash: Hash,
    pub is_sunset: bool,
    pub item_type: i32,
    pub item_sub_type: i32,
}

impl From<InventoryArmor> for StrippedInventoryArmor {
    fn from(i: InventoryArmor) -> Self {
        Self {
            id: i.id,
            item_instance_id: i.item_instance_id,
            masterworked: i.masterworked,
            may_be_bugged: i.may_be_bugged,
            mobility: i.mobility,
            resilience: i.resilience,
            recovery: i.recovery,
            discipline: i.discipline,
            intellect: i.intellect,
            strength: i.strength,
            energy_level: i.energy_level,
            energy_affinity: i.energy_affinity,
            hash: i.hash,
            slot: i.slot,
            clazz: i.clazz,
            perk: i.perk,
            is_exotic: i.is_exotic,
            rarity: i.rarity,
            exotic_perk_hash: i.exotic_perk_hash,
            is_sunset: i.is_sunset,
            item_type: i.item_type,
            item_sub_type: i.item_sub_type
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct ArmorInformation {
    icon: String,
    watermark: String,
    name: String,
}

impl From<InventoryArmor> for ArmorInformation {
    fn from(i: InventoryArmor) -> Self {
        Self {
            icon: i.icon,
            watermark: i.watermark_icon,
            name: i.name,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct InventoryArmor {
    pub id: i32,
    pub item_instance_id: Hash,
    pub masterworked: bool,
    pub may_be_bugged: bool, // if there was an error in the parsing
    pub mobility: u8,
    pub resilience: u8,
    pub recovery: u8,
    pub discipline: u8,
    pub intellect: u8,
    pub strength: u8,
    pub energy_level: u8,
    pub energy_affinity: DestinyEnergyType,
    pub stat_plug_hashes: Vec<Hash>, // or Vec<Option<Hash>>
    pub hash: Hash,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub watermark_icon: String,
    pub slot: ArmorSlot,
    pub clazz: CharacterClass,
    pub perk: ArmorPerkOrSlot,
    pub is_exotic: bool,
    pub rarity: TierType,
    pub exotic_perk_hash: Hash,
    pub armor2: bool,
    pub is_sunset: bool,
    pub raw_data: Option<String>,
    pub item_type: i32,
    pub item_sub_type: i32,
    pub investment_stats: Vec<DestinyItemInvestmentStatDefinition>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ManifestArmor {
    pub hash: Hash,
    pub slot: ArmorSlot,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct DestinyItemInvestmentStatDefinition {
    pub stat_type_hash: Hash,
    pub value: i32,
    pub is_conditionally_active: bool,
}

#[derive(Copy, Clone, Default, Deserialize, Eq, PartialEq)]
pub enum TierType {
    #[default]
    Unknown = 0,
    Currency = 1,
    Basic = 2,
    Common = 3,
    Rare = 4,
    Superior = 5,
    Exotic = 6,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub energy: DestinyEnergyType,
    pub energyLevel: (), // que?
    pub hash: Hash,
    pub item_instance_id: Hash,
    pub name: String,
    pub exotic: bool,
    pub masterworked: bool,
    pub may_be_bugged: bool,
    pub slot: (),
    pub perk: (),
    pub transfer_state: (),
    pub stats: Stats,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CharacterClass {
    None = -1,
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
}

#[derive(Deserialize, Serialize)]
pub struct ThreadConfig {
    pub count: usize,
    pub current: usize,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatModifier {
    None,
    MinorMobility,
    MajorMobility,
    MinorResilience,
    MajorResilience,
    MinorRecovery,
    MajorRecovery,
    MinorDiscipline,
    MajorDiscipline,
    MinorIntellect,
    MajorIntellect,
    MinorStrength,
    MajorStrength,
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ArmorSlot {
    ArmorSlotNone,
    ArmorSlotHelmet,
    ArmorSlotGauntlet,
    ArmorSlotChest,
    ArmorSlotLegs,
    ArmorSlotClass,
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ArmorStat {
    Mobility,
    Resilience,
    Recovery,
    Discipline,
    Intellect,
    Strength
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub enum ExoticChoiceModel {
    All,
    None,
    Some(Hash),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerConfig {
    pub character_class: CharacterClass,
    pub add_constant_1_resilience: bool,
    pub disabled_items: Vec<Hash>,
    pub minimum_stat_tiers: HashMap<ArmorStat, FixableSelection<i32>>,
    pub maximum_stat_mods: i32,
    pub maximum_mod_slots: HashMap<ArmorSlot, FixableSelection<i32>>,
    pub allow_blue_armor_pieces: bool,
    pub ignore_sunset_armor: bool,
    pub assume_legendaries_masterworked: bool,
    pub assume_exotics_masterworked: bool,
    pub assume_class_item_masterworked: bool,
    pub only_use_masterworked_items: bool,
    pub limit_parsed_results: bool,
    pub try_limit_wasted_stats: bool,
    pub only_show_results_with_no_wasted_stats: bool,
    pub show_wasted_stats_column: bool,
    pub show_potential_tier_column: bool,
    pub selected_mod_element: ModifierType,
    pub enabled_mods: Vec<ModOrAbility>,
    pub selected_exotic: ExoticChoiceModel,
    pub armor_affinities: HashMap<ArmorSlot, FixableSelection<DestinyEnergyType>>,
    pub armor_perks: HashMap<ArmorSlot, FixableSelection<ArmorPerkOrSlot>>,
    pub ignore_armor_affinities_on_masterworked_items: bool,
    pub ignore_armor_affinities_on_non_masterworked_items: bool,
}

#[derive(Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum ArmorPerkOrSlot {
    None,
    SlotNightmare,
    SlotArtificer,
    SlotLastWish,
    SlotGardenOfSalvation,
    SlotDeepStoneCrypt,
    SlotVaultOfGlass,
    PerkIronBanner,
    PerkUniformedOfficer,
    SlotVowOfTheDisciple,
    SlotKingsFall,
    PerkPlunderersTrappings,
    // #[serde(rename = "COUNT")]
    Count,
}

#[derive(Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum DestinyEnergyType {
    Any = 0,
    Arc = 1,
    Thermal = 2,
    Void = 3,
    Ghost = 4,
    Subclass = 5,
    Stasis = 6,
}

#[derive(Deserialize, Serialize)]
pub enum ModOrAbility {

    PowerfulFriends,
    RadiantLight,

    // Negative mods
    ProtectiveLight = 100,
    ExtraReserves,
    PreciselyCharged,
    StacksOnStacks,
    PrecisionCharge,
    SurpriseAttack,
    EnergyConverter,
    ChargeHarvester,

    WhisperOfDurance = 1000,
    WhisperOfChains,
    WhisperOfConduction,
    WhisperOfShards,

    WhisperOfHedrons = 1100,
    WhisperOfBonds,
    WhisperOfHunger,
    WhisperOfFractures,

    // VOID
    EchoOfExpulsion = 1200,
    EchoOfProvision,
    EchoOfPersistence,
    EchoOfLeeching,
    EchoOfDomineering,
    EchoOfDilation,
    EchoOfUndermining,

    EchoOfInstability, //+10 str
    EchoOfHarvest,
    EchoOfObscurity, //+10rec
    EchoOfStarvation,

    // SOLAR
    EmberOfBenelovence = 1300,
    EmberOfBeams,
    EmberOfEmpyrean,
    EmberOfCombustion,
    EmberOfChar,
    EmberOfTempering,
    EmberOfEruption,
    EmberOfWonder,
    EmberOfSearing,
    //EchoOfExchange ,
    //EchoOfRemnants,
    //EchoOfReprisal,

    SparkOfBrilliance = 1400,
    SparkOfFeedback,
    SparkOfDischarge,
    SparkOfFocus,
    SparkOfVolts,
    SparkOfResistance,
    SparkOfShock,
}

#[derive(Deserialize, Serialize)]
pub enum ModifierType {
    CombatStyleMod,
    Stasis,
    Void,
    Solar,

}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixableSelection<T> {
    pub fixed: bool,
    pub value: T
}

pub type Stats = [u8; 6];
