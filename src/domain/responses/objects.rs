use serde::Serialize;
use crate::domain::persistence::models::UrlPair;
use crate::domain::types::objects;

#[derive(Serialize)]
pub struct UrlPairResponse {
    pub long_url: objects::LongUrl,
    pub short_url: objects::ShortUrl,
}

impl From<UrlPair> for UrlPairResponse {
    fn from(p: UrlPair) -> Self {
        UrlPairResponse {
            long_url: p.long_url,
            short_url: p.short_url,
        }
    }
}