use serde::Deserialize;
use crate::domain::types::objects;

#[allow(non_snake_case)]
#[derive(Deserialize, utoipa::ToSchema)]
pub(crate) struct ShortenUrlRequest {
    pub(crate) longUrl: objects::LongUrl,
}