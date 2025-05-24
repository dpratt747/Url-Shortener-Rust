use serde::Deserialize;
use crate::domain::types::url;

#[allow(non_snake_case)]
#[derive(Deserialize, utoipa::ToSchema)]
pub(crate) struct ShortenUrlRequest {
    pub(crate) longUrl: url::LongUrl,
}