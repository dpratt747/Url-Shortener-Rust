use crate::persistence::database::DatabaseAlg;
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use std::collections::HashMap;

pub trait UrlShortenerServiceAlg {
    fn store_long_url_and_get_short_url(&mut self, long_url: String) -> String;

    fn get_all(&self) -> HashMap<String, String>;

    fn get_long_url_with_short(&self, short_url: String) -> Option<String>;
}

pub struct UrlShortenerService {
    db: Box<dyn DatabaseAlg>,
}

impl UrlShortenerServiceAlg for UrlShortenerService {
    fn store_long_url_and_get_short_url(&mut self, long_url: String) -> String {
        let short_url_path = Self::generate_alphanumeric_string(5);
        self.db.store(long_url, short_url_path.to_string());
        short_url_path
    }

    fn get_all(&self) -> HashMap<String, String> {
        self.db.get_all()
    }

    fn get_long_url_with_short(&self, short_url: String) -> Option<String> {
        self.db.get_long_url_with_short_url(short_url)
    }
}

impl UrlShortenerService {
    fn generate_alphanumeric_string(len: usize) -> String {
        let rng = rng();
        rng.sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub fn new(database: Box<dyn DatabaseAlg>) -> Self {
        UrlShortenerService { db: database }
    }
}
