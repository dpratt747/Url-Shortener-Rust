use crate::persistence::database::DatabaseAlg;
use crate::domain::types::objects;
use crate::domain::errors::domain_errors;
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use crate::domain::persistence::models::UrlPair;

pub trait UrlShortenerServiceAlg {
    fn store_long_url_and_get_short_url(&mut self, long_url: objects::LongUrl) -> Result<objects::ShortUrl, domain_errors::ServiceError>;

    fn get_all(&self) -> Result<Vec<UrlPair>, domain_errors::ServiceError>;

    fn get_long_url_with_short(&self, short_url: objects::ShortUrl) -> Result<Option<objects::LongUrl>, domain_errors::ServiceError>;
}

pub struct UrlShortenerService {
    db: Box<dyn DatabaseAlg>,
}

impl UrlShortenerServiceAlg for UrlShortenerService {
    fn store_long_url_and_get_short_url(&mut self, long_url: objects::LongUrl) -> Result<objects::ShortUrl, domain_errors::ServiceError> {
        let short_url_path = objects::ShortUrl(Self::generate_alphanumeric_string(5));
        self.db.store(long_url, short_url_path.clone())?; // the '?' unwraps the result it exits the function if the result is an error
        Ok(short_url_path)
    }

    fn get_all(&self) -> Result<Vec<UrlPair>, domain_errors::ServiceError> {
        Ok(self.db.get_all()?)
    }

    fn get_long_url_with_short(&self, short_url: objects::ShortUrl) -> Result<Option<objects::LongUrl>, domain_errors::ServiceError> {
        Ok(self.db.get_long_url_with_short_url(short_url)?)
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
