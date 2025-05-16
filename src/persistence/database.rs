use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use utoipa::openapi::KnownFormat::Duration;
use utoipa::PartialSchema;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, utoipa::ToSchema)]
pub(crate) struct ShortUrl(pub(crate) String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, utoipa::ToSchema)]
pub(crate) struct LongUrl(pub(crate) String);

pub trait DatabaseAlg: Send + Sync {
    fn store(&mut self, long_url: LongUrl, short_url: ShortUrl);
    fn get_all(&self) -> HashMap<LongUrl, ShortUrl>;
    fn get_long_url_with_short_url(&self, short_url: ShortUrl) -> Option<LongUrl>;
}

impl DatabaseAlg for Arc<Mutex<InMemoryDatabase>> {
    fn store(&mut self, long_url: LongUrl, short_url: ShortUrl) {
        self.lock().unwrap().store(long_url, short_url);
    }

    fn get_all(&self) -> HashMap<LongUrl, ShortUrl> {
        self.lock().unwrap().get_all()
    }

    fn get_long_url_with_short_url(&self, short_url: ShortUrl) -> Option<LongUrl> {
        self.lock().unwrap().get_long_url_with_short_url(short_url)
    }
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
        self.store
            .clone()
            .into_iter()
            .map(|(long_url, (short_url, _))| (long_url, short_url))
            .collect()
    }

    fn get_long_url_with_short_url(&self, short_url: ShortUrl) -> Option<LongUrl> {
        self.store.iter().find_map(|(key, (url, dt))| {
            if url == &short_url {
                Some(key.clone())
            } else {
                None
            }
        })
    }
}

impl InMemoryDatabase {
    // like a companion object
    pub fn new(in_memory_store: HashMap<LongUrl, (ShortUrl, DateTime<Local>)>) -> Self {
        InMemoryDatabase {
            store: in_memory_store,
        }
    }

    fn is_30_mins_or_older_local(dt: &DateTime<Local>) -> bool {
        let now = Local::now();
        let thirty_minutes = chrono::Duration::minutes(30);
        now.signed_duration_since(dt) >= thirty_minutes
    }
}
