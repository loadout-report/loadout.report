use std::collections::{HashMap, HashSet};
use std::ops::RangeBounds;
use std::time::Duration;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use rexie::{KeyRange, TransactionMode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use data::api::manifest::model::Hash;
use crate::db::build_database;
use crate::model::{ArmorInformation, ArmorPerkOrSlot, ArmorSlot, ArmorStat, CharacterClass, DestinyEnergyType, ExoticChoiceModel, InventoryArmor, Item, ManifestArmor, ModifierValue, StatMod, ModGroup, Msg, StatModifier, Stats, StrippedInventoryArmor, ThreadConfig, TierType, WorkerConfig};

pub struct ArmorWorker {
    db: Option<rexie::Rexie>,
    ready: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub current_class: CharacterClass,
    pub config: WorkerConfig,
    pub thread_split: ThreadConfig,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExoticResult {
    icon: String,
    watermark: String,
    name: String,
    hash: Hash,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResult {
    exotic: Vec<ExoticResult>,
    mod_count: usize,
    mod_cost: i32,
    mods: Vec<StatModifier>,
    stats: Stats,
    stats_no_mods: Stats,
    tiers: u8,
    waste: u8,
    items: [Vec<Item>; 4],
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Runtime {
    maximum_possible_tiers: Stats,
    stat_combo_3x_100: HashSet<()>,
    stat_combo_4x_100: HashSet<()>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputStats {
    permutation_count: usize,
    item_count: usize,
    total_time: Duration,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    runtime: Runtime,
    results: Vec<ItemResult>,
    total: Option<usize>,
    done: bool,
    stats: Option<OutputStats>,
}

impl Worker for ArmorWorker {
    type Message = Msg<()>;
    type Input = Input;
    type Output = Output;

    fn create(scope: &WorkerScope<Self>) -> Self {
        {
            scope.send_future(async {
                let db = build_database().await.unwrap();
                Msg::Ready(db)
            });
        }


        Self {
            db: None,
            ready: false,
        }
    }

    fn update(&mut self, _: &WorkerScope<Self>, msg: Self::Message) {
        if let Msg::Ready(db) = msg {
            self.db = Some(db);
            self.ready = true;
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        // run job
        if !self.ready {
            // todo: respond with error
            return;
        }
    }
}

async fn run_job(scope: &WorkerScope<ArmorWorker>, db: rexie::Rexie, data: Input) -> Result<(), Box<dyn std::error::Error>> {
    let config = data.config;

    let selected_exotic = if let ExoticChoiceModel::Some(exotic) = config.selected_exotic {
        let tx = db.transaction(&["manifestArmor"], TransactionMode::ReadOnly)?;
        let manifest_armor = tx.store("manifestArmor").unwrap();
        let armor = manifest_armor.index("hash").unwrap().get(&JsValue::from(exotic)).await?;
        let armor: ManifestArmor = serde_wasm_bindgen::from_value(armor)?;
        tx.done().await?;
        Some(armor)
    } else {
        None
    };

    // todo: single iteration to split vectors
    let armor: Vec<InventoryArmor> = {
        let tx = db.transaction(&["inventoryArmor"], TransactionMode::ReadOnly)?;
        let inventory_armor = tx.store("inventory_armor")?;
        let key_range = KeyRange::only(&(config.character_class as u32).into()).unwrap();
        let armor = inventory_armor.index("clazz").unwrap();
        let armor = armor.get_all(Some(&key_range), None, None, None).await?;
        armor.iter().into_iter()
            .map(|(_, value)| serde_wasm_bindgen::from_value(value.clone()).unwrap())
            .collect()
    };

    let armor_info: HashMap<Hash, ArmorInformation> = armor.iter()
        .map(|i| (i.hash, Into::into(i.clone())))
        .collect();

    let armor: Vec<StrippedInventoryArmor> = armor.iter()
        .map(|i| Into::into(i.clone()))
        .collect();

    let armor: Vec<_> = armor.iter()
        .filter(|i| i.slot != ArmorSlot::ArmorSlotNone)
        .filter(|i| !config.disabled_items.contains(&i.item_instance_id))
        .filter(|i| config.selected_exotic != ExoticChoiceModel::None || !i.is_exotic)
        .filter(|i| config.selected_exotic == ExoticChoiceModel::All || selected_exotic.is_some_and(|e| e.slot != i.slot || e.hash == i.hash))
        .filter(|i| !config.only_use_masterworked_items || i.masterworked)
        .filter(|i| config.allow_blue_armor_pieces || i.rarity == TierType::Exotic || i.rarity == TierType::Superior)
        .filter(|i| !config.ignore_sunset_armor || !i.is_sunset)
        .filter(|i| {
            let affinity = config.armor_affinities.get(&i.slot).unwrap();
            !affinity.fixed ||
                affinity.value == DestinyEnergyType::Any ||
                (i.masterworked && config.ignore_armor_affinities_on_masterworked_items) ||
                (!i.masterworked && config.ignore_armor_affinities_on_non_masterworked_items) ||
                affinity.value == i.energy_affinity
        })
        .filter(|i| {
            let perks = config.armor_perks.get(&i.slot).unwrap();
            i.is_exotic ||
                !perks.fixed ||
                perks.value == ArmorPerkOrSlot::None ||
                perks.value == i.perk
        })
        .collect();

    let (mut helmets, mut gauntlets, mut chests, mut legs, class_items): (
        Vec<StrippedInventoryArmor>,
        Vec<StrippedInventoryArmor>,
        Vec<StrippedInventoryArmor>,
        Vec<StrippedInventoryArmor>,
        Vec<StrippedInventoryArmor>,
    ) = armor.iter().fold(Default::default(), |mut t, i| {
        match i.slot {
            ArmorSlot::ArmorSlotNone => (),
            ArmorSlot::ArmorSlotHelmet => t.0.push(**i),
            ArmorSlot::ArmorSlotGauntlet => t.1.push(**i),
            ArmorSlot::ArmorSlotChest => t.2.push(**i),
            ArmorSlot::ArmorSlotLegs => t.3.push(**i),
            ArmorSlot::ArmorSlotClass => t.4.push(**i),
        }
        t
    });

    // threading
    if data.thread_split.count > 1 {
        let sets: [usize; 4] = [helmets.len(), gauntlets.len(), chests.len(), legs.len()];
        let max = sets.iter().max().unwrap();
        let index = sets.iter().position(|l| l == max).unwrap();
        let keep_length = max / data.thread_split.count;
        let start_index = keep_length * data.thread_split.current;
        let mut end_index = keep_length * (data.thread_split.current + 1);
        if keep_length * data.thread_split.count != *max && data.thread_split.current == data.thread_split.count - 1 {
            end_index += max - keep_length * data.thread_split.count;
        }

        match index {
            0 => {
                helmets.drain(end_index..);
                helmets.drain(0..start_index);
            }
            1 => {
                gauntlets.drain(end_index..);
                gauntlets.drain(0..start_index);
            }
            2 => {
                chests.drain(end_index..);
                chests.drain(0..start_index);
            }
            3 => {
                legs.drain(end_index..);
                legs.drain(0..start_index);
            }
            _ => unreachable!(),
        }
    }

    let available_class_item_perk_types: HashSet<_> = class_items.iter()
        .map(|i| i.perk)
        .collect();

    let available_class_item_energy_perk_dict = available_class_item_perk_types.iter()
        .fold(HashMap::new(), |mut map, apos| {
            if !map.contains_key(apos) {
                map.insert(apos, HashSet::new());
            }
            if !map.contains_key(&ArmorPerkOrSlot::None) {
                map.insert(&ArmorPerkOrSlot::None, HashSet::new());
            }
            class_items.iter()
                .filter(|i| i.perk == apos)
                .for_each(|i| {
                    map.get_mut(&ArmorPerkOrSlot::None).unwrap().insert(i.energy_affinity);
                    map.get_mut(apos).unwrap().insert(i.energy_affinity);
                });
            map
        });

    // console.debug

    let mut runtime: Runtime = Runtime {
        maximum_possible_tiers: [0, 0, 0, 0, 0, 0],
        stat_combo_3x_100: Default::default(),
        stat_combo_4x_100: Default::default()
    };

    let constant_bonus = prepare_constant_stat_bonus(config.clone());
    let constant_element_requirement = prepare_constant_element_requirement(config.clone());


    todo!()
}

fn prepare_constant_modslot_requirement(config: WorkerConfig) {
    for i in 0..12 {

    }
}

fn prepare_constant_stat_bonus(config: WorkerConfig) -> Stats {
    let mut constant_bonus: Stats = Default::default();
    for stat_mod in config.enabled_mods {
        let modifiers = get_modifiers(stat_mod);
        constant_bonus = modifiers.apply(constant_bonus, config.character_class);
    }
    constant_bonus
}

fn prepare_constant_element_requirement(config: WorkerConfig) -> [u8; 7] {
    let mut cer: [u8; 7] = Default::default();
    cer[config.armor_affinities.get(&ArmorSlot::ArmorSlotHelmet).unwrap().value as usize] += 1;
    cer[config.armor_affinities.get(&ArmorSlot::ArmorSlotChest).unwrap().value as usize] += 1;
    cer[config.armor_affinities.get(&ArmorSlot::ArmorSlotGauntlet).unwrap().value as usize] += 1;
    cer[config.armor_affinities.get(&ArmorSlot::ArmorSlotLegs).unwrap().value as usize] += 1;

    let class_item_affinity = config.armor_affinities.get(&ArmorSlot::ArmorSlotClass).unwrap();
    if !class_item_affinity.fixed {
        cer[class_item_affinity.value as usize] += 1;
    }
    cer[0] = 0;
    cer
}

fn get_modifiers(moa: StatMod) -> ModGroup {
    match moa {
        StatMod::PowerfulFriends => ModGroup::single(ArmorStat::Mobility, 20),
        StatMod::RadiantLight => ModGroup::single(ArmorStat::Strength, 20),
        StatMod::ProtectiveLight => ModGroup::single(ArmorStat::Strength, -10),
        StatMod::ExtraReserves => ModGroup::single(ArmorStat::Intellect, -10),
        StatMod::PreciselyCharged => ModGroup::single(ArmorStat::Discipline, -10),
        StatMod::StacksOnStacks => ModGroup::single(ArmorStat::Recovery, -10),
        StatMod::PrecisionCharge => ModGroup::single(ArmorStat::Strength, -10),
        StatMod::SurpriseAttack => ModGroup::single(ArmorStat::Intellect, -10),
        StatMod::EnergyConverter => ModGroup::single(ArmorStat::Discipline, -10),
        StatMod::ChargeHarvester => ModGroup::single(ArmorStat::ClassAbilityRegenerationStat, -10),
        // stasis mods
        StatMod::WhisperOfDurance => ModGroup::single(ArmorStat::Strength, 10),
        StatMod::WhisperOfChains => ModGroup::single(ArmorStat::Recovery, 10),
        StatMod::WhisperOfShards => ModGroup::single(ArmorStat::Resilience, 10),
        StatMod::WhisperOfConduction => ModGroup::Double(
            ModifierValue::new(ArmorStat::Resilience, 10),
            ModifierValue::new(ArmorStat::Intellect, 10),
        ),
        StatMod::WhisperOfBonds => ModGroup::Double(
            ModifierValue::new(ArmorStat::Discipline, -10),
            ModifierValue::new(ArmorStat::Intellect, -10),
        ),
        StatMod::WhisperOfHedrons => ModGroup::single(ArmorStat::Strength, -10),
        StatMod::WhisperOfFractures => ModGroup::single(ArmorStat::Discipline, -10),
        StatMod::WhisperOfHunger => ModGroup::Double(
            ModifierValue::new(ArmorStat::Mobility, -10),
            ModifierValue::new(ArmorStat::Recovery, -10),
        ),
        StatMod::EchoOfExpulsion => ModGroup::single(ArmorStat::Intellect, -10),
        StatMod::EchoOfProvision => ModGroup::single(ArmorStat::Strength, -10),
        StatMod::EchoOfPersistence => ModGroup::single(ArmorStat::ClassAbilityRegenerationStat, -10),
        StatMod::EchoOfLeeching => ModGroup::single(ArmorStat::Resilience, 10),
        StatMod::EchoOfDomineering => ModGroup::single(ArmorStat::Discipline, 10),
        StatMod::EchoOfDilation => ModGroup::Double(
            ModifierValue::new(ArmorStat::Mobility, 10),
            ModifierValue::new(ArmorStat::Intellect, 10),
        ),
        StatMod::EchoOfUndermining => ModGroup::single(ArmorStat::Discipline, -20),
        StatMod::EchoOfInstability => ModGroup::single(ArmorStat::Strength, 10),
        StatMod::EchoOfHarvest => ModGroup::single(ArmorStat::Intellect, -10),
        StatMod::EchoOfObscurity => ModGroup::single(ArmorStat::Recovery, 10),
        StatMod::EchoOfStarvation => ModGroup::single(ArmorStat::Recovery, -10),
        StatMod::EmberOfBenelovence => ModGroup::single(ArmorStat::Discipline, -10),
        StatMod::EmberOfBeams => ModGroup::single(ArmorStat::Intellect, 10),
        StatMod::EmberOfEmpyrean => ModGroup::single(ArmorStat::Resilience, -10),
        StatMod::EmberOfCombustion => ModGroup::single(ArmorStat::Strength, 10),
        StatMod::EmberOfChar => ModGroup::single(ArmorStat::Discipline, 10),
        StatMod::EmberOfTempering => ModGroup::single(ArmorStat::Recovery, -10),
        StatMod::EmberOfEruption => ModGroup::single(ArmorStat::Strength, 10),
        StatMod::EmberOfWonder => ModGroup::single(ArmorStat::Resilience, 10),
        StatMod::EmberOfSearing => ModGroup::single(ArmorStat::Recovery, 10),
        StatMod::SparkOfBrilliance => ModGroup::single(ArmorStat::Intellect, 10),
        StatMod::SparkOfFeedback => ModGroup::single(ArmorStat::Resilience, 10),
        StatMod::SparkOfDischarge => ModGroup::single(ArmorStat::Strength, -10),
        StatMod::SparkOfFocus => ModGroup::single(ArmorStat::ClassAbilityRegenerationStat, -10),
        StatMod::SparkOfVolts => ModGroup::single(ArmorStat::Recovery, 10),
        StatMod::SparkOfResistance => ModGroup::single(ArmorStat::Strength, 10),
        StatMod::SparkOfShock => ModGroup::single(ArmorStat::Discipline, -10),
    }

}
