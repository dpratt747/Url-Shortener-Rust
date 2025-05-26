use crate::domain::errors::domain_errors;
use crate::domain::persistence::models::GetUrlPair;
use crate::domain::types::objects;
use crate::persistence::database::DatabaseAlg;
use async_trait::async_trait;
use rand::distr::Alphanumeric;
use rand::Rng;

#[async_trait]
pub trait UrlShortenerServiceAlg {
    async fn store_long_url_and_get_short_url(
        &self,
        long_url: objects::LongUrl,
    ) -> Result<objects::ShortUrl, domain_errors::ServiceError>;
    async fn get_all(&self) -> Result<Vec<GetUrlPair>, domain_errors::ServiceError>;
    async fn get_long_url_with_short(
        &self,
        short_url: objects::ShortUrl,
    ) -> Result<Option<objects::LongUrl>, domain_errors::ServiceError>;
}

pub struct UrlShortenerService {
    db: Box<dyn DatabaseAlg>,
}

#[async_trait]
impl UrlShortenerServiceAlg for UrlShortenerService {
    async fn store_long_url_and_get_short_url(
        &self,
        long_url: objects::LongUrl,
    ) -> Result<objects::ShortUrl, domain_errors::ServiceError> {
        let short_url_path = objects::ShortUrl(Self::generate_alphanumeric_string(5));
        self.db.store(long_url, short_url_path.clone()).await?;
        Ok(short_url_path)
    }

    async fn get_all(&self) -> Result<Vec<GetUrlPair>, domain_errors::ServiceError> {
        Ok(self.db.get_all().await?)
    }

    async fn get_long_url_with_short(
        &self,
        short_url: objects::ShortUrl,
    ) -> Result<Option<objects::LongUrl>, domain_errors::ServiceError> {
        Ok(self.db.get_long_url_with_short_url(short_url).await?)
    }
}

impl UrlShortenerService {
    fn generate_alphanumeric_string(len: usize) -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub fn new(database: Box<dyn DatabaseAlg>) -> Self {
        UrlShortenerService { db: database }
    }
}
