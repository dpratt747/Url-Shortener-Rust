use serde::Serialize;
use crate::domain::persistence::models::UrlPair;
use crate::domain::types::url;

#[derive(Debug, Serialize)]
pub struct UrlPairResponse {
    pub long_url: url::LongUrl,
    pub short_url: url::ShortUrl,
}

impl From<UrlPair> for UrlPairResponse {
    fn from(p: UrlPair) -> Self {
        UrlPairResponse {
            long_url: p.long_url,
            short_url: p.short_url,
        }
    }
}