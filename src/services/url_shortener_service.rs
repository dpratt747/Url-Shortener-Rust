use crate::persistence::database::{DatabaseAlg, LongUrl, ShortUrl};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use std::collections::HashMap;

pub trait UrlShortenerServiceAlg {
    fn store_long_url_and_get_short_url(&mut self, long_url: LongUrl) -> ShortUrl;

    fn get_all(&self) -> HashMap<LongUrl, ShortUrl>;

    fn get_long_url_with_short(&self, short_url: ShortUrl) -> Option<LongUrl>;
}

pub struct UrlShortenerService {
    db: Box<dyn DatabaseAlg>,
}

impl UrlShortenerServiceAlg for UrlShortenerService {
    fn store_long_url_and_get_short_url(&mut self, long_url: LongUrl) -> ShortUrl {
        let short_url_path = ShortUrl(Self::generate_alphanumeric_string(5));
        self.db.store(long_url, short_url_path.clone());
        short_url_path
    }

    fn get_all(&self) -> HashMap<LongUrl, ShortUrl> {
        self.db.get_all_within_cutoff_time()
    }

    fn get_long_url_with_short(&self, short_url: ShortUrl) -> Option<LongUrl> {
        self.db.get_long_url_with_short_url_within_cutoff_time(short_url)
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
