use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, utoipa::ToSchema)]
pub(crate) struct ShortUrl(pub(crate) String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, utoipa::ToSchema)]
pub(crate) struct LongUrl(pub(crate) String);

pub trait DatabaseAlg: Send + Sync {
    fn store(&mut self, long_url: LongUrl, short_url: ShortUrl);
    fn get_all(&self) -> HashMap<LongUrl, ShortUrl>;
    fn get_long_url_with_short_url(&self, short_url: ShortUrl) -> Option<LongUrl>;
}

#[derive(Clone)]
pub struct InMemoryDatabase {
    store: HashMap<LongUrl, (ShortUrl, DateTime<Local>)>,
}

impl DatabaseAlg for InMemoryDatabase {
    fn store(&mut self, long_url: LongUrl, short_url: ShortUrl) -> () {
        // save datetime.now
        self.store.insert(long_url, (short_url, Local::now()));
    }

    fn get_all(&self) -> HashMap<LongUrl, ShortUrl> {
        let cutoff = Local::now() - Self::CUTOFF_DURATION;

        self.store
            .clone()
            .into_iter()
            .filter(|(_, (_, timestamp))| *timestamp >= cutoff)
            .map(|(long_url, (short_url, _))| (long_url, short_url))
            .collect()
    }

    fn get_long_url_with_short_url(&self, short_url: ShortUrl) -> Option<LongUrl> {
        let cutoff = Local::now() - Self::CUTOFF_DURATION;

        self.store
            .iter()
            .filter(|(_, (_, timestamp))| *timestamp >= cutoff)
            .find_map(|(key, (url, _))| {
                if url == &short_url {
                    Some(key.clone())
                } else {
                    None
                }
            })
    }
}

impl InMemoryDatabase {
    const CUTOFF_DURATION: Duration = Duration::minutes(30);

    // like a companion object
    pub fn new(in_memory_store: HashMap<LongUrl, (ShortUrl, DateTime<Local>)>) -> Self {
        InMemoryDatabase {
            store: in_memory_store,
        }
    }
}
