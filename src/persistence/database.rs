use crate::domain::types::url;
use crate::schema::urls::dsl::urls;
use crate::schema::urls::{long_url, short_url};

use chrono::{DateTime, Local};
use diesel::prelude::*;
use diesel::PgConnection;
use std::collections::HashMap;
use std::sync::Arc;
use diesel::r2d2::{ConnectionManager, Pool};

pub trait DatabaseAlg: Send + Sync {
    fn store(&mut self, long_url_value: url::LongUrl, short_url_value: url::ShortUrl);
    fn get_all(&self) -> Vec<(url::LongUrl, url::ShortUrl)>;
    fn get_long_url_with_short_url(&self, short_url: url::ShortUrl) -> Option<url::LongUrl>;
}

#[derive(Clone)]
pub struct InMemoryDatabase {
    store: HashMap<url::LongUrl, (url::ShortUrl, DateTime<Local>)>,
}

// #[derive(Clone)]
pub struct UrlDatabase {
    connection: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl DatabaseAlg for UrlDatabase {
    fn store(&mut self, long_url_value: url::LongUrl, short_url_value: url::ShortUrl) {
        todo!()
    }

    fn get_all(&self) -> Vec<(url::LongUrl, url::ShortUrl)> {
        let mut conn = self.connection.get().expect("couldn't get connection from pool");
        
        urls.select((long_url, short_url))
            .load::<(url::LongUrl, url::ShortUrl)>(&mut conn)
            .expect("couldn't get urls")
    }

    fn get_long_url_with_short_url(&self, short_url_value: url::ShortUrl) -> Option<url::LongUrl> {
        todo!()
    }
}

impl UrlDatabase {
    // like a companion object
    pub fn new(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self { connection: conn }
    }
}

impl DatabaseAlg for InMemoryDatabase {
    fn store(&mut self, long_url_value: url::LongUrl, short_url_value: url::ShortUrl) -> () {
        // save datetime.now
        self.store
            .insert(long_url_value, (short_url_value, Local::now()));
    }

    fn get_all(&self) -> Vec<(url::LongUrl, url::ShortUrl)> {
        self.store
            .clone()
            .iter()
            .map(|(k, (v, _))| (k.clone(), v.clone()))
            .collect()
    }

    fn get_long_url_with_short_url(&self, short_url_value: url::ShortUrl) -> Option<url::LongUrl> {
        self.store.iter().find_map(|(key, (url, _))| {
            if url == &short_url_value {
                Some(key.clone())
            } else {
                None
            }
        })
    }
}

impl InMemoryDatabase {
    // like a companion object
    pub fn new(in_memory_store: HashMap<url::LongUrl, (url::ShortUrl, DateTime<Local>)>) -> Self {
        Self {
            store: in_memory_store,
        }
    }
}
