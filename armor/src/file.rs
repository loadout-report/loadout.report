#[cfg(not(feature = "wasm"))]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::model::{CharacterClass, ExoticChoiceModel, ModifierType, WorkerConfig};

    #[test]
    fn test_compute() {
        let config: WorkerConfig = WorkerConfig {
            character_class: CharacterClass::Titan,
            add_constant_1_resilience: false,
            disabled_items: vec![],
            minimum_stat_tiers: Default::default(),
            minimum_stat_tiers_: Default::default(),
            maximum_mod_slots: Default::default(),
            maximum_mod_slots_: Default::default(),
            armor_affinities: Default::default(),
            armor_affinities_: Default::default(),
            armor_perks: Default::default(),
            armor_perks_: Default::default(),
            allow_blue_armor_pieces: false,
            ignore_sunset_armor: false,
            assume_legendaries_masterworked: true,
            assume_exotics_masterworked: true,
            assume_class_item_masterworked: true,
            only_use_masterworked_items: false,
            limit_parsed_results: false,
            try_limit_wasted_stats: false,
            use_fotl_armor: false,
            only_show_results_with_no_wasted_stats: false,
            show_wasted_stats_column: false,
            show_potential_tier_column: false,
            // selected_mod_element: ModifierType::CombatStyleMod,
            enabled_mods: vec![],
            selected_exotic: ExoticChoiceModel::All,
            ignore_armor_affinities_on_masterworked_items: true,
            ignore_armor_affinities_on_non_masterworked_items: false,
        };
        let armor = include_bytes!("../data/d2ap_armor.json");
        let manifest = include_bytes!("../data/d2ap_manifest.json");

        let armor = serde_json::from_slice(armor).unwrap();
        let manifest = serde_json::from_slice(manifest).unwrap();
    }

}
