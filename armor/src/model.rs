use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::once;
use std::ops::{Add, AddAssign};
use gloo_worker::HandlerId;
use num_derive::FromPrimitive;
use data::api::manifest::model::Hash;
use serde::{Deserialize, Serialize};
use stats::Stats;

pub mod stats;

pub enum Msg<T> {
    Respond { output: T, id: HandlerId },
    Done,
    Ready(rexie::Rexie),
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct StrippedInventoryArmor {
    pub id: i32,
    // may not need this
    pub item_instance_id: Hash,
    pub masterworked: bool,
    pub may_be_bugged: bool,
    // may not need this
    // if there was an error in the parsing
    pub stats: Stats,

    pub energy_level: u8,
    pub energy_affinity: DestinyEnergyType,
    pub hash: Hash,
    pub slot: ArmorSlot,
    pub clazz: CharacterClass,
    pub perk: ArmorPerkOrSlot,
    pub is_exotic: bool,
    pub rarity: TierType,
    pub exotic_perk_hash: Hash,
    // may not need this
    pub is_sunset: bool,
    pub item_type: i32,
    pub item_sub_type: i32,
}

impl StrippedInventoryArmor {
    pub fn base_stats(self) -> Stats {
        self.stats
    }

    pub fn stats(self) -> Stats {
        if self.masterworked {
            self.base_stats().modify_all(2)
        } else {
            self.base_stats()
        }
    }

    pub fn assume_masterwork(mut self) -> Self {
        self.masterworked = true;
        self
    }
}

impl From<InventoryArmor> for StrippedInventoryArmor {
    fn from(i: InventoryArmor) -> Self {
        Self {
            id: i.id,
            item_instance_id: i.item_instance_id,
            masterworked: i.masterworked,
            may_be_bugged: i.may_be_bugged,
            stats: Stats::new([i.mobility, i.resilience, i.recovery, i.discipline, i.intellect, i.strength]),
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
    pub icon: String,
    pub watermark: String,
    pub name: String,
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

pub struct InventoryArmorComponents(ArmorInformation, StrippedInventoryArmor);

impl From<InventoryArmor> for InventoryArmorComponents {
    fn from(armor: InventoryArmor) -> Self {
        let info = ArmorInformation {
            icon: armor.icon,
            watermark: armor.watermark_icon,
            name: armor.name,
        };
        let item = StrippedInventoryArmor {
            id: armor.id,
            item_instance_id: armor.item_instance_id,
            masterworked: armor.masterworked,
            may_be_bugged: armor.may_be_bugged,
            stats: Stats::new([
                armor.mobility, armor.resilience, armor.recovery,
                armor.discipline, armor.intellect, armor.strength
            ]),
            energy_level: armor.energy_level,
            energy_affinity: armor.energy_affinity,
            hash: armor.hash,
            slot: armor.slot,
            clazz: armor.clazz,
            perk: armor.perk,
            is_exotic: armor.is_exotic,
            rarity: armor.rarity,
            exotic_perk_hash: armor.exotic_perk_hash,
            is_sunset: armor.is_sunset,
            item_type: armor.item_type,
            item_sub_type: armor.item_sub_type,
        };
        InventoryArmorComponents(info, item)
    }
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

#[derive(Copy, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
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
    pub energyLevel: u8,
    // que?
    pub hash: Hash,
    pub item_instance_id: Hash,
    pub name: String,
    pub exotic: bool,
    pub masterworked: bool,
    pub may_be_bugged: bool,
    pub slot: ArmorSlot,
    pub perk: ArmorPerkOrSlot,
    pub transfer_state: u8,
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

#[derive(Copy, Clone, Serialize, Deserialize, FromPrimitive)]
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
pub enum SimpleArmorStat {
    Mobility,
    Resilience,
    Recovery,
    Discipline,
    Intellect,
    Strength,
}

impl SimpleArmorStat {
    pub fn for_class(class: CharacterClass) -> Self {
        match class {
            CharacterClass::None => panic!("no class provided"),
            CharacterClass::Titan => SimpleArmorStat::Resilience,
            CharacterClass::Hunter => SimpleArmorStat::Mobility,
            CharacterClass::Warlock => SimpleArmorStat::Recovery,
        }
    }
}

impl From<(ArmorStat, CharacterClass)> for SimpleArmorStat {
    fn from((stat, class): (ArmorStat, CharacterClass)) -> Self {
        match stat {
            ArmorStat::ClassAbilityRegenerationStat => {
                match class {
                    CharacterClass::None => panic!("uh-oh"), // todo: error handling
                    CharacterClass::Titan => SimpleArmorStat::Resilience,
                    CharacterClass::Hunter => SimpleArmorStat::Mobility,
                    CharacterClass::Warlock => SimpleArmorStat::Recovery,
                }
            }
            ArmorStat::Mobility => SimpleArmorStat::Mobility,
            ArmorStat::Resilience => SimpleArmorStat::Resilience,
            ArmorStat::Recovery => SimpleArmorStat::Recovery,
            ArmorStat::Discipline => SimpleArmorStat::Discipline,
            ArmorStat::Intellect => SimpleArmorStat::Intellect,
            ArmorStat::Strength => SimpleArmorStat::Strength
        }
    }
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
    pub minimum_stat_tiers: HashMap<SimpleArmorStat, FixableSelection<u8>>,
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

#[derive(Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash, Default)]
pub enum ArmorPerkOrSlot {
    #[default]
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

#[derive(Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash, Default)]
pub enum DestinyEnergyType {
    #[default]
    Any = 0,
    Arc = 1,
    Thermal = 2,
    Void = 3,
    Ghost = 4,
    Subclass = 5,
    Stasis = 6,
}

impl DestinyEnergyType {
    pub(crate) fn to_simple(self) -> usize {
        match self {
            DestinyEnergyType::Any => 0,
            DestinyEnergyType::Arc => 1,
            DestinyEnergyType::Thermal => 2,
            DestinyEnergyType::Void => 3,
            DestinyEnergyType::Stasis => 4,
            _ => 0,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ArmorSet {
    pub helmet: StrippedInventoryArmor,
    pub gauntlets: StrippedInventoryArmor,
    pub chest: StrippedInventoryArmor,
    pub legs: StrippedInventoryArmor,
}

impl ArmorSet {
    pub fn new(
        helmet: StrippedInventoryArmor,
        gauntlets: StrippedInventoryArmor,
        chest: StrippedInventoryArmor,
        legs: StrippedInventoryArmor,
    ) -> Self {
        Self {
            helmet,
            gauntlets,
            chest,
            legs,
        }
    }

    pub fn exotic(&self) -> Option<StrippedInventoryArmor> {
        if self.helmet.is_exotic {
            Some(self.helmet)
        } else if self.gauntlets.is_exotic {
            Some(self.gauntlets)
        } else if self.legs.is_exotic {
            Some(self.legs)
        } else if self.chest.is_exotic {
            Some(self.chest)
        } else {
            None
        }
    }

    pub fn stat_total(&self) -> Stats {
        self.iter()
            .map(|i| i.stats())
            .reduce(|acc, e| acc + e).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item=StrippedInventoryArmor> {
        once(self.helmet)
            .chain(once(self.gauntlets))
            .chain(once(self.chest))
            .chain(once(self.legs))
    }
}

impl IntoIterator for ArmorSet {
    type Item = StrippedInventoryArmor;
    type IntoIter = impl Iterator<Item=StrippedInventoryArmor>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
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
    Single(SimpleModifierValue),
    Double(SimpleModifierValue, SimpleModifierValue),
}

impl ModGroup {
    pub fn single(stat: SimpleArmorStat, value: i8) -> Self {
        ModGroup::Single(SimpleModifierValue { stat, value })
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SimpleModifierValue {
    pub stat: SimpleArmorStat,
    pub value: i8,
}

impl SimpleModifierValue {
    pub fn new(stat: SimpleArmorStat, value: i8) -> Self {
        Self {
            stat,
            value,
        }
    }
}

impl From<(ModifierValue, CharacterClass)> for SimpleModifierValue {
    fn from((m, c): (ModifierValue, CharacterClass)) -> Self {
        Self {
            stat: From::from((m.stat, c)),
            value: m.value,
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


