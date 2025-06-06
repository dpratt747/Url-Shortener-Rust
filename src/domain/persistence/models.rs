use crate::domain::types::objects;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::persistence::schema::urls_within_designated_mins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertUrls {
    pub long_url: objects::LongUrl,
    pub short_url: objects::ShortUrl,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::persistence::schema::urls_within_designated_mins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetUrlPair {
    pub long_url: objects::LongUrl,
    pub short_url: objects::ShortUrl,
}
