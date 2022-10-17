use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub type Hash = i64;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    pub destiny_inventory_item_definition: HashMap<String, InventoryItem>
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundColor {
    pub color_hash: Hash,
    pub red: i32,
    pub green: i32,
    pub blue: i32,
    pub alpha: i32,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub max_stack_size: i32,
    pub bucket_type_hash: Hash,
    pub recovery_bucket_type_hash: Hash,
    pub tier_type_hash: Option<Hash>,
    pub is_instance_item: bool,
    pub non_transferrable_original: bool,
    pub tier_type_name: Option<String>,
    pub tier_type: i32,
    pub expiration_tooltip: Option<String>,
    pub expired_in_activity_message: Option<String>,
    pub expired_in_orbit_message: Option<String>,
    pub suppress_expiration_when_objectives_complete: bool,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perk {
    pub requirement_display_string: String,
    pub perk_hash: Hash,
    pub perk_visibility: i32,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plug {
    pub plug_category_identifier: String,
    pub plug_category_hash: Hash,
    // todo: more fields
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayProperties {
    pub description: String,
    pub name: String,
    pub icon: Option<String>,
    pub has_icon: bool,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub display_properties: DisplayProperties,
    // tooltip_notifications:
    pub background_color: Option<BackgroundColor>,
    pub screenshot: Option<String>,
    pub item_type_display_name: Option<String>,
    pub flavor_text: Option<String>,
    pub ui_item_display_style: Option<String>,
    pub item_type_and_tier_display_name: Option<String>,
    pub display_source: Option<String>,
    pub inventory: Inventory,
    pub plug: Option<Plug>,
    pub acquire_reward_site_hash: Hash,
    pub acquire_unlock_hash: Hash,
    // investment_stats:
    pub perks: Option<Vec<Perk>>,
    pub summary_item_hash: Option<Hash>,
    pub allow_actions: bool,
    pub does_postmaster_pull_have_side_effects: bool,
    pub non_transferrable: bool,
    pub item_category_hashes: Option<Vec<Hash>>,
    pub special_item_type: i32,
    pub item_type: i32,
    pub class_type: i32,
    pub breaker_type: i32,
    pub equippable: bool,
    pub default_damage_type: i32,
    pub is_wrapper: bool,
    pub hash: Hash,
    pub index: i32,
    pub redacted: bool,
    pub blacklisted: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub label: String,
    pub icon: String,
    pub rarity: i32,
    pub categories: Vec<Hash>,
    pub hash: Hash,
}

impl From<InventoryItem> for Item {
    fn from(item: InventoryItem) -> Self {
        Item {
            label: item.display_properties.name,
            icon: item.display_properties.icon.map(|url| "https://www.bungie.net".to_string() + url.as_str()).unwrap_or_default(),
            rarity: item.inventory.tier_type,
            categories: item.item_category_hashes.unwrap_or_default(),
            hash: item.hash,
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDefinition {
    pub short_title: String,
    pub deprecated: bool,
    pub visible: bool,
    pub display_properties: DisplayProperties,
    pub grant_destiny_breaker_type: i32,
    pub grant_destiny_item_type: i32,
    pub grant_destiny_sub_type: i32,
    pub grant_destiny_class: i32,
    pub grouped_category_hashes: Vec<Hash>,
    pub is_plug: bool,
    pub parent_category_hashes: Vec<Hash>,
    pub group_category_only: bool,
    pub hash: Hash,
    pub index: i32,
    pub redacted: bool,
    pub blacklisted: bool,
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub label: String,
    pub description: String,
    pub short_label: String,
    pub hash: Hash,
}

impl From<CategoryDefinition> for Category {
    fn from(category: CategoryDefinition) -> Self {
        Category {
            label: category.display_properties.name,
            short_label: category.short_title,
            description: category.display_properties.description,
            hash: category.hash,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.label.clone())
    }
}