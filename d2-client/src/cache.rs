use std::sync::{Arc};
use lru_time_cache::LruCache;
use std::sync::Mutex;
use crate::generated::models::destiny::responses::DestinyProfileResponse;

#[derive(Clone)]
pub struct Cache {
    pub(crate) profile: Arc<Mutex<LruCache<Membership, DestinyProfileResponse>>>,
    pub(crate) manifest: manifest::Cache,
}

impl Cache {
    pub fn new() -> Self {
        let ttl = std::time::Duration::from_secs(60 * 60 * 12);
        let max = 10000;
        Self {
            profile: Arc::new(
                Mutex::new(
                    LruCache::with_expiry_duration_and_capacity(ttl, max)
                )
            ),
            manifest: Default::default(),
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache::new()
    }
}

mod manifest {
    use std::collections::HashMap;
    use std::future::Future;
    use std::sync::Arc;
    use std::time::{SystemTime};
    use chrono::{DateTime, Utc};
    use std::sync::Mutex;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use crate::generated::models::destiny::config::DestinyManifest;
    use crate::generated::models::destiny::definitions::*;
    use crate::generated::models::destiny::definitions::activity_modifiers::DestinyActivityModifierDefinition;
    use crate::generated::models::destiny::definitions::breaker_types::DestinyBreakerTypeDefinition;
    use crate::generated::models::destiny::definitions::collectibles::DestinyCollectibleDefinition;
    use crate::generated::models::destiny::definitions::items::DestinyItemTierTypeDefinition;
    use crate::generated::models::destiny::definitions::loadouts::{DestinyLoadoutColorDefinition, DestinyLoadoutConstantsDefinition, DestinyLoadoutIconDefinition, DestinyLoadoutNameDefinition};
    use crate::generated::models::destiny::definitions::metrics::DestinyMetricDefinition;
    use crate::generated::models::destiny::definitions::milestones::DestinyMilestoneDefinition;
    use crate::generated::models::destiny::definitions::power_caps::DestinyPowerCapDefinition;
    use crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition;
    use crate::generated::models::destiny::definitions::records::DestinyRecordDefinition;
    use crate::generated::models::destiny::definitions::seasons::{DestinyEventCardDefinition, DestinySeasonDefinition, DestinySeasonPassDefinition};
    use crate::generated::models::destiny::definitions::sockets::{DestinyPlugSetDefinition, DestinySocketCategoryDefinition, DestinySocketTypeDefinition};
    use crate::generated::models::destiny::definitions::traits::DestinyTraitDefinition;
    use crate::generated::models::destiny::entities::items::DestinyItemComponent;
    use crate::id::Id;

    #[derive(Clone, Default)]
    pub struct Cache {
        pub(crate) definitions: Arc<Mutex<Definitions>>,
        pub(crate) content_paths: Arc<Mutex<Option<DestinyManifest>>>,
    }

    #[derive(Clone, Error, Debug)]
    pub enum LoadError {
        #[error("No loader set")]
        NoLoader,
        #[error("Unknown error")]
        Unknown
    }

    pub trait Loader<T> {
        async fn load(&self) -> Result<HashMap<Id<T>, T>, LoadError>;
    }

    #[derive(Clone, Default, Serialize, Deserialize)]
    pub struct DefinitionTable<T>
        where T: Clone + Serialize + Deserialize
    {
        pub data: Arc<Mutex<Option<HashMap<Id<T>, T>>>>,
        async_loader: Option<Box<dyn Loader<T>>>,
    }

    impl DefinitionTable<T> {
        pub fn new() -> Self {
            Self {
                data: Default::default(),
                async_loader: None,
            }
        }

        pub fn get(&self, id: &Id<T>) -> Option<T> {
            self.data.get(id).cloned()
        }

        pub fn set_loader(&mut self, loader: Box<dyn Loader<T>>) {
            self.async_loader = Some(loader);
        }

        pub async fn load(&mut self) -> Result<(), LoadError> {
            if Some(loader) = self.async_loader.as_ref() {
                let data = loader.load().await?;
                self.data.lock().unwrap().replace(data);
                Ok(())
            } else {
                Err(LoadError::NoLoader)
            }
        }
    }

    impl <T> From<T> for DefinitionTable<T> {
        fn from(t: T) -> Self {
            Self {
                data: Arc::new(Mutex::new(Some(t))),
                async_loader: None,
            }
        }
    }

    #[derive(Clone, Default, Serialize, Deserialize)]
    pub struct Definitions {
        /// NodeStepSummary
        /// ArtDyeChannel
        /// ArtDyeReference
        /// Place
        pub place: DefinitionTable<DestinyPlaceDefinition>,
        /// Activity
        pub activity: DefinitionTable<DestinyActivityDefinition>,
        /// ActivityType
        pub activity_type: DefinitionTable<DestinyActivityTypeDefinition>,
        /// Class
        pub class: DefinitionTable<DestinyClassDefinition>,
        /// Gender
        pub gender: DefinitionTable<DestinyGenderDefinition>,
        /// InventoryBucket
        pub inventory_bucket: DefinitionTable<DestinyInventoryBucketDefinition>,
        /// Race
        pub race: DefinitionTable<DestinyRaceDefinition>,
        /// TalentGrid
        /// Unlock
        /// SandboxPerk
        pub sandbox_perk: DefinitionTable<DestinySandboxPerkDefinition>,
        /// StatGroup
        pub stat_group: DefinitionTable<DestinyStatGroupDefinition>,
        /// ProgressionMapping
        /// Faction
        pub faction: DefinitionTable<DestinyFactionDefinition>,
        /// VendorGroup
        pub vendor_group: DefinitionTable<DestinyVendorGroupDefinition>,
        /// RewardSource
        /// UnlockValue
        /// RewardMapping
        /// RewardSheet
        /// ItemCategory
        pub item_category: DefinitionTable<DestinyItemCategoryDefinition>,
        /// DamageType
        pub damage_type: DefinitionTable<DestinyDamageTypeDefinition>,
        /// ActivityMode
        pub activity_mode: DefinitionTable<DestinyActivityModeDefinition>,
        /// MedalTier
        /// Achievement
        /// ActivityGraph
        /// ActivityInteractable
        /// Bond
        /// CharacterCustomizationCategory
        /// CharacterCustomizationOption
        /// Collectible
        pub collectible: DefinitionTable<DestinyCollectibleDefinition>,
        /// Destination
        pub destination: DefinitionTable<DestinyDestinationDefinition>,
        /// EntitlementOffer
        /// EquipmentSlot
        /// EventCard
        pub event_card: DefinitionTable<DestinyEventCardDefinition>,
        /// Stat
        pub stat: DefinitionTable<DestinyStatDefinition>,
        /// InventoryItem
        pub inventory_item: DefinitionTable<DestinyInventoryItemDefinition>,
        /// InventoryItemLite
        /// ItemTierType
        pub item_tier_type: DefinitionTable<DestinyItemTierTypeDefinition>,
        /// LoadoutColor
        pub loadout_color: DefinitionTable<DestinyLoadoutColorDefinition>,
        /// LoadoutIcon
        pub loadout_icon: DefinitionTable<DestinyLoadoutIconDefinition>,
        /// LoadoutName
        pub loadout_name: DefinitionTable<DestinyLoadoutNameDefinition>,
        /// Location
        pub location: DefinitionTable<DestinyLocationDefinition>,
        /// Lore
        /// MaterialRequirementSet
        pub material_requirement_set: DefinitionTable<DestinyMaterialRequirementSetDefinition>,
        /// Metric
        pub metric: DefinitionTable<DestinyMetricDefinition>,
        /// Objective
        pub objective: DefinitionTable<DestinyObjectiveDefinition>,
        /// PlatformBucketMapping
        /// PlugSet
        pub plug_set: DefinitionTable<DestinyPlugSetDefinition>,
        /// PowerCap
        pub power_cap: DefinitionTable<DestinyPowerCapDefinition>,
        /// PresentationNode
        pub presentation_node: DefinitionTable<DestinyPresentationNodeDefinition>,
        /// Progression
        pub progression: DefinitionTable<DestinyProgressionDefinition>,
        /// ProgressionLevelRequirement
        /// Record
        pub record: DefinitionTable<DestinyRecordDefinition>,
        /// RewardAdjusterPointer
        /// RewardAdjusterProgressionMap
        /// RewardItemList
        /// SackRewardItemList
        /// SandboxPattern
        /// Season
        pub season: DefinitionTable<DestinySeasonDefinition>,
        /// SeasonPass
        pub season_pass: DefinitionTable<DestinySeasonPassDefinition>,
        /// SocialCommendation
        /// SocketCategory
        pub socket_category: DefinitionTable<DestinySocketCategoryDefinition>,
        /// SocketType
        pub socket_type: DefinitionTable<DestinySocketTypeDefinition>,
        /// Trait
        pub r#trait: DefinitionTable<DestinyTraitDefinition>,
        /// UnlockCountMapping
        /// UnlockEvent
        /// UnlockExpressionMapping
        /// Vendor
        pub vendor: DefinitionTable<DestinyVendorDefinition>,
        /// Milestone
        pub milestone: DefinitionTable<DestinyMilestoneDefinition>,
        /// ActivityModifier
        pub activity_modifier: DefinitionTable<DestinyActivityModifierDefinition>,
        /// ReportReasonCategory
        /// Artifact
        /// BreakerType
        pub breaker_type: DefinitionTable<DestinyBreakerTypeDefinition>,
        /// Checklist
        /// EnergyType
        /// SocialCommendationNode
        /// GuardianRank
        /// GuardianRankConstants
        /// LoadoutConstants
        pub loadout_constants: DefinitionTable<DestinyLoadoutConstantsDefinition>,
    }

    impl Definitions {
        pub fn new() -> Self {
            Default::default()
        }

        pub async fn load_all(&mut self) {

        }
    }

}
