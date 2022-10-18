use std::collections::HashMap;
use std::convert::TryFrom;
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
    pub may_be_bugged: bool,
    // if there was an error in the parsing
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
            item_sub_type: i.item_sub_type,
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
    pub may_be_bugged: bool,
    // if there was an error in the parsing
    pub mobility: u8,
    pub resilience: u8,
    pub recovery: u8,
    pub discipline: u8,
    pub intellect: u8,
    pub strength: u8,
    pub energy_level: u8,
    pub energy_affinity: DestinyEnergyType,
    pub stat_plug_hashes: Vec<Hash>,
    // or Vec<Option<Hash>>
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
    pub energyLevel: (),
    // que?
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
    Strength,
    ClassAbilityRegenerationStat = 10,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Copy, Clone)]
pub enum ExoticChoiceModel {
    All,
    None,
    Some(Hash),
}

impl ExoticChoiceModel {
    pub fn is(self, hash: Hash) -> bool {
        match self {
            ExoticChoiceModel::All => false,
            ExoticChoiceModel::None => false,
            ExoticChoiceModel::Some(h) => h == hash,
        }
    }

    pub fn some(self) -> bool {
        match self {
            ExoticChoiceModel::All => false,
            ExoticChoiceModel::None => false,
            ExoticChoiceModel::Some(_) => true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkerConfig {
    pub character_class: CharacterClass,
    pub add_constant_1_resilience: bool,
    pub disabled_items: Vec<Hash>,
    pub minimum_stat_tiers: HashMap<ArmorStat, FixableSelection<i32>>,
    pub maximum_stat_mods: i32,
    pub maximum_mod_slots: HashMap<ArmorSlot, FixableSelection<u8>>,
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
    pub enabled_mods: Vec<StatMod>,
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

impl TryFrom<usize> for ArmorPerkOrSlot {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == ArmorPerkOrSlot::None as usize => Ok(ArmorPerkOrSlot::None),
            x if x == ArmorPerkOrSlot::SlotNightmare as usize => Ok(ArmorPerkOrSlot::SlotNightmare),
            x if x == ArmorPerkOrSlot::SlotArtificer as usize => Ok(ArmorPerkOrSlot::SlotArtificer),
            x if x == ArmorPerkOrSlot::SlotLastWish as usize => Ok(ArmorPerkOrSlot::SlotLastWish),
            x if x == ArmorPerkOrSlot::SlotGardenOfSalvation as usize => Ok(ArmorPerkOrSlot::SlotGardenOfSalvation),
            x if x == ArmorPerkOrSlot::SlotDeepStoneCrypt as usize => Ok(ArmorPerkOrSlot::SlotDeepStoneCrypt),
            x if x == ArmorPerkOrSlot::SlotVaultOfGlass as usize => Ok(ArmorPerkOrSlot::SlotVaultOfGlass),
            x if x == ArmorPerkOrSlot::PerkIronBanner as usize => Ok(ArmorPerkOrSlot::PerkIronBanner),
            x if x == ArmorPerkOrSlot::PerkUniformedOfficer as usize => Ok(ArmorPerkOrSlot::PerkUniformedOfficer),
            x if x == ArmorPerkOrSlot::SlotVowOfTheDisciple as usize => Ok(ArmorPerkOrSlot::SlotVowOfTheDisciple),
            x if x == ArmorPerkOrSlot::SlotKingsFall as usize => Ok(ArmorPerkOrSlot::SlotKingsFall),
            x if x == ArmorPerkOrSlot::PerkPlunderersTrappings as usize => Ok(ArmorPerkOrSlot::PerkPlunderersTrappings),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum DestinyEnergyType {
    Any = 0,
    Arc = 1,
    Thermal = 2,
    Void = 3,
    Ghost = 4,
    Subclass = 5,
    Stasis = 6,
}

#[derive(Deserialize, Serialize, Copy, Clone, Hash, Eq, PartialEq)]
pub enum StatMod {
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

    EchoOfInstability,
    //+10 str
    EchoOfHarvest,
    EchoOfObscurity,
    //+10rec
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

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ModGroup {
    Single(ModifierValue),
    Double(ModifierValue, ModifierValue),
}

impl ModGroup {
    pub fn single(stat: ArmorStat, value: i8) -> Self {
        ModGroup::Single(ModifierValue { stat, value })
    }

    pub fn apply(self, stats: Stats, class: CharacterClass) -> Stats {
        match self {
            ModGroup::Single(val) => val.apply(stats, class),
            ModGroup::Double(v1, v2) => v1.apply(v2.apply(stats, class), class)
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ModifierValue {
    pub stat: ArmorStat,
    pub value: i8,
}

impl ModifierValue {
    pub fn new(stat: ArmorStat, value: i8) -> Self {
        Self {
            stat,
            value,
        }
    }

    pub fn apply(self, mut stats: Stats, class: CharacterClass) -> Stats {
        let val = self.value as i16;
        match self.stat {
            ArmorStat::Mobility => stats[0] += val,
            ArmorStat::Resilience => stats[1] += val,
            ArmorStat::Recovery => stats[2] += val,
            ArmorStat::Discipline => stats[3] += val,
            ArmorStat::Intellect => stats[4] += val,
            ArmorStat::Strength => stats[5] += val,
            ArmorStat::ClassAbilityRegenerationStat => match class {
                CharacterClass::None => panic!("uh-oh"),
                CharacterClass::Titan => stats[1] += val,
                CharacterClass::Hunter => stats[0] += val,
                CharacterClass::Warlock => stats[2] += val,
            }
        };
        stats
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Hash, Eq, PartialEq)]
pub enum ModifierType {
    CombatStyleMod,
    Stasis,
    Void,
    Solar,

}

#[derive(Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixableSelection<T> {
    pub fixed: bool,
    pub value: T,
}

pub type Stats = [i16; 6];

