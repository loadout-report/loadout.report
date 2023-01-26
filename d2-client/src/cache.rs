use std::sync::{Arc};
use lru_time_cache::LruCache;
use rustgie::types::destiny::responses::DestinyProfileResponse;
use tokio::sync::Mutex;
use data::api::model::profile::{Membership, ProfileStruct};

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
    use std::sync::Arc;
    use std::time::{SystemTime};
    use chrono::{DateTime, Utc};
    use tokio::sync::Mutex;

    #[derive(Clone, Default)]
    pub struct Cache {
        pub(crate) metadata: Arc<Mutex<Metadata>>,
        pub(crate) items: Arc<Mutex<Vec<data::api::manifest::model::Item>>>,
        pub(crate) categories: Arc<Mutex<Vec<data::api::manifest::model::Category>>>,
        pub(crate) content_paths: Arc<Mutex<Option<data::api::manifest::Components>>>,
    }

    #[derive(Copy, Clone, Default)]
    pub struct Metadata {
        pub(crate) last_modified: Option<DateTime<Utc>>,
        pub(crate) uuids: Uuids,
    }

    #[derive(Copy, Clone, Default)]
    pub struct Uuids {
        pub(crate) items: Option<uuid::Uuid>,
        pub(crate) categories: Option<uuid::Uuid>,
    }

}
