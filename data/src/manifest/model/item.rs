use serde::Deserialize;

use super::*;

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundColor {
    colorHash: Hash,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    max_stack_size: i32,
    bucket_type_hash: Hash,
    recovery_bucket_type_hash: Hash,
    tier_type_hash: Hash,
    is_instance_item: bool,
    non_transferrable_original: bool,
    tier_type_name: String,
    tier_type: i32,
    expiration_tooltip: String,
    expired_in_activity_message: String,
    expired_in_orbit_message: String,
    suppress_expiration_when_objectives_complete: bool,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perk {
    requirement_display_string: String,
    perk_hash: Hash,
    perk_visibility: i32,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plug {
    plug_category_identifier: String,
    plug_category_hash: Hash,
    // todo: more fields
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    display_properties: DisplayProperties,
    // tooltip_notifications:
    background_color: BackgroundColor,
    screenshot: String,
    item_type_display_name: String,
    flavor_text: String,
    ui_item_display_style: String,
    item_type_and_tier_display_name: String,
    display_source: String,
    inventory: Inventory,
    plug: Plug,
    acquire_reward_site_hash: Hash,
    acquire_unlock_hash: Hash,
    // investment_stats:
    perks: Vec<Perk>,
    summary_item_hash: Hash,
    allow_actions: bool,
    does_postmaster_pull_have_side_effects: bool,
    non_transferrable: bool,
    item_category_hashes: Vec<Hash>,
    special_item_type: i32,
    item_type: i32,
    class_type: i32,
    breaker_type: i32,
    equippable: bool,
    default_damage_type: i32,
    is_wrapper: bool,
    hash: Hash,
    index: i32,
    redacted: bool,
    blacklisted: bool,
}