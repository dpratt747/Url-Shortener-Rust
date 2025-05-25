use diesel::prelude::*;
use serde::Serialize;
use crate::domain::types::objects;


#[derive(Insertable)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct InsertUrls {
    pub long_url: objects::LongUrl,
    pub short_url: objects::ShortUrl
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Urls {
    pub id: i32,
    pub long_url: objects::LongUrl,
    pub short_url: objects::ShortUrl,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Queryable, Serialize)]
pub struct UrlPair {
    pub long_url: objects::LongUrl,
    pub short_url: objects::ShortUrl
}
