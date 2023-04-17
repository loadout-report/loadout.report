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
    use std::future::{Future, join};
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

    pub trait Loader<N, T> {
        async fn load(&self) -> Result<HashMap<Id<N, T>, T>, LoadError>;
    }

    #[derive(Clone, Default, Serialize, Deserialize)]
    pub struct DefinitionTable<N, T>
        where T: Clone + Serialize + Deserialize
    {
        pub data: Arc<Mutex<Option<HashMap<Id<N, T>, T>>>>,
        dirty: bool,
        async_loader: Option<Box<dyn Loader<N, T>>>,
    }

    impl<N, T> DefinitionTable<N, T> {
        pub fn new() -> Self {
            Self {
                data: Default::default(),
                dirty: true,
                async_loader: None,
            }
        }

        pub fn get(&self, id: &Id<N, T>) -> Option<T> {
            self.data.get(id).cloned()
        }

        pub fn set_loader(&mut self, loader: Box<dyn Loader<N, T>>) {
            self.async_loader = Some(loader);
            dirty = true;
        }

        pub fn has_loader(&self) -> bool {
            self.async_loader.is_some()
        }

        async fn load(&mut self) -> Result<(), LoadError> {
            if Some(loader) = self.async_loader.as_ref() {
                let data = loader.load().await?;
                self.data.lock().unwrap().replace(data);
                dirty = false;
                Ok(())
            } else {
                Err(LoadError::NoLoader)
            }
        }

        pub async fn load_if_dirty(&mut self) -> Result<(), LoadError> {
            if dirty {
                self.load().await
            } else {
                Ok(())
            }
        }

        pub async fn load_if_possible(&mut self) -> Result<(), LoadError> {
            if dirty && self.has_loader() {
                self.load().await
            } else {
                Ok(())
            }
        }

    }

    impl <T> From<T> for DefinitionTable<N, T> {
        fn from(t: T) -> Self {
            Self {
                data: Arc::new(Mutex::new(Some(t))),
                dirty: false,
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
        pub place: DefinitionTable<u32, DestinyPlaceDefinition>,
        /// Activity
        pub activity: DefinitionTable<u32, DestinyActivityDefinition>,
        /// ActivityType
        pub activity_type: DefinitionTable<u32, DestinyActivityTypeDefinition>,
        /// Class
        pub class: DefinitionTable<u32, DestinyClassDefinition>,
        /// Gender
        pub gender: DefinitionTable<u32, DestinyGenderDefinition>,
        /// InventoryBucket
        pub inventory_bucket: DefinitionTable<u32, DestinyInventoryBucketDefinition>,
        /// Race
        pub race: DefinitionTable<u32, DestinyRaceDefinition>,
        /// TalentGrid
        /// Unlock
        /// SandboxPerk
        pub sandbox_perk: DefinitionTable<u32, DestinySandboxPerkDefinition>,
        /// StatGroup
        pub stat_group: DefinitionTable<u32, DestinyStatGroupDefinition>,
        /// ProgressionMapping
        /// Faction
        pub faction: DefinitionTable<u32, DestinyFactionDefinition>,
        /// VendorGroup
        pub vendor_group: DefinitionTable<u32, DestinyVendorGroupDefinition>,
        /// RewardSource
        /// UnlockValue
        /// RewardMapping
        /// RewardSheet
        /// ItemCategory
        pub item_category: DefinitionTable<u32, DestinyItemCategoryDefinition>,
        /// DamageType
        pub damage_type: DefinitionTable<u32, DestinyDamageTypeDefinition>,
        /// ActivityMode
        pub activity_mode: DefinitionTable<u32, DestinyActivityModeDefinition>,
        /// MedalTier
        /// Achievement
        /// ActivityGraph
        /// ActivityInteractable
        /// Bond
        /// CharacterCustomizationCategory
        /// CharacterCustomizationOption
        /// Collectible
        pub collectible: DefinitionTable<u32, DestinyCollectibleDefinition>,
        /// Destination
        pub destination: DefinitionTable<u32, DestinyDestinationDefinition>,
        /// EntitlementOffer
        /// EquipmentSlot
        /// EventCard
        pub event_card: DefinitionTable<u32, DestinyEventCardDefinition>,
        /// Stat
        pub stat: DefinitionTable<u32, DestinyStatDefinition>,
        /// InventoryItem
        pub inventory_item: DefinitionTable<u32, DestinyInventoryItemDefinition>,
        /// InventoryItemLite
        /// ItemTierType
        pub item_tier_type: DefinitionTable<u32, DestinyItemTierTypeDefinition>,
        /// LoadoutColor
        pub loadout_color: DefinitionTable<u32, DestinyLoadoutColorDefinition>,
        /// LoadoutIcon
        pub loadout_icon: DefinitionTable<u32, DestinyLoadoutIconDefinition>,
        /// LoadoutName
        pub loadout_name: DefinitionTable<u32, DestinyLoadoutNameDefinition>,
        /// Location
        pub location: DefinitionTable<u32, DestinyLocationDefinition>,
        /// Lore
        /// MaterialRequirementSet
        pub material_requirement_set: DefinitionTable<u32, DestinyMaterialRequirementSetDefinition>,
        /// Metric
        pub metric: DefinitionTable<u32, DestinyMetricDefinition>,
        /// Objective
        pub objective: DefinitionTable<u32, DestinyObjectiveDefinition>,
        /// PlatformBucketMapping
        /// PlugSet
        pub plug_set: DefinitionTable<u32, DestinyPlugSetDefinition>,
        /// PowerCap
        pub power_cap: DefinitionTable<u32, DestinyPowerCapDefinition>,
        /// PresentationNode
        pub presentation_node: DefinitionTable<u32, DestinyPresentationNodeDefinition>,
        /// Progression
        pub progression: DefinitionTable<u32, DestinyProgressionDefinition>,
        /// ProgressionLevelRequirement
        /// Record
        pub record: DefinitionTable<u32, DestinyRecordDefinition>,
        /// RewardAdjusterPointer
        /// RewardAdjusterProgressionMap
        /// RewardItemList
        /// SackRewardItemList
        /// SandboxPattern
        /// Season
        pub season: DefinitionTable<u32, DestinySeasonDefinition>,
        /// SeasonPass
        pub season_pass: DefinitionTable<u32, DestinySeasonPassDefinition>,
        /// SocialCommendation
        /// SocketCategory
        pub socket_category: DefinitionTable<u32, DestinySocketCategoryDefinition>,
        /// SocketType
        pub socket_type: DefinitionTable<u32, DestinySocketTypeDefinition>,
        /// Trait
        pub r#trait: DefinitionTable<u32, DestinyTraitDefinition>,
        /// UnlockCountMapping
        /// UnlockEvent
        /// UnlockExpressionMapping
        /// Vendor
        pub vendor: DefinitionTable<u32, DestinyVendorDefinition>,
        /// Milestone
        pub milestone: DefinitionTable<u32, DestinyMilestoneDefinition>,
        /// ActivityModifier
        pub activity_modifier: DefinitionTable<u32, DestinyActivityModifierDefinition>,
        /// ReportReasonCategory
        /// Artifact
        /// BreakerType
        pub breaker_type: DefinitionTable<u32, DestinyBreakerTypeDefinition>,
        /// Checklist
        /// EnergyType
        /// SocialCommendationNode
        /// GuardianRank
        /// GuardianRankConstants
        /// LoadoutConstants
        pub loadout_constants: DefinitionTable<u32, DestinyLoadoutConstantsDefinition>,
    }

    impl Definitions {
        pub fn new() -> Self {
            Default::default()
        }

        pub async fn load_all(&mut self) {
            tokio::join!(
                self.inventory_item.load_if_possible()
            );
            // todo: other loaders
        }
    }

}
