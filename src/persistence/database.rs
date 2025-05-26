use crate::domain::errors::domain_errors;
use crate::domain::persistence::models;
use crate::domain::types::objects;
use crate::schema::valid_urls::dsl::valid_urls as valid_urls_table;
use crate::schema::valid_urls::{long_url as long_url_column, short_url as short_url_column};

use crate::domain::persistence::models::GetUrlPair;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::task::spawn_blocking;

#[async_trait]
pub trait DatabaseAlg: Send + Sync {
    async fn store(
        &self,
        long_url_value: objects::LongUrl,
        short_url_value: objects::ShortUrl,
    ) -> Result<(), domain_errors::StorageError>;
    async fn get_all(&self) -> Result<Vec<GetUrlPair>, domain_errors::StorageError>;
    async fn get_long_url_with_short_url(
        &self,
        short_url: objects::ShortUrl,
    ) -> Result<Option<objects::LongUrl>, domain_errors::StorageError>;
}

pub struct UrlDatabase {
    connection: Arc<Pool<ConnectionManager<PgConnection>>>,
}

#[async_trait]
impl DatabaseAlg for UrlDatabase {
    async fn store(
        &self,
        long_url_value: objects::LongUrl,
        short_url_value: objects::ShortUrl,
    ) -> Result<(), domain_errors::StorageError> {
        let conn = self.connection.clone();
        spawn_blocking(move || {
            let mut conn = conn.get()
                .map_err(|e| domain_errors::StorageError::ConnectionFailed(e.to_string()))?;

            let insert_url = models::InsertUrls {
                long_url: long_url_value,
                short_url: short_url_value,
            };

            diesel::insert_into(valid_urls_table)
                .values(&insert_url)
                .execute(&mut conn)
                .map_err(|err| domain_errors::StorageError::from(err))?;

            Ok(())
        }).await
        .map_err(|e| domain_errors::StorageError::TaskJoinError(e.to_string()))?
    }

    async fn get_all(&self) -> Result<Vec<GetUrlPair>, domain_errors::StorageError> {
        let conn = self.connection.clone();
        spawn_blocking(move || {
            let mut conn = conn.get()
                .map_err(|e| domain_errors::StorageError::ConnectionFailed(e.to_string()))?;

            valid_urls_table.select(GetUrlPair::as_select())
                .load(&mut conn)
                .map_err(|err| domain_errors::StorageError::SelectionFailed(err.to_string()))
        }).await
        .map_err(|e| domain_errors::StorageError::TaskJoinError(e.to_string()))?
    }

    async fn get_long_url_with_short_url(
        &self,
        short_url_value: objects::ShortUrl,
    ) -> Result<Option<objects::LongUrl>, domain_errors::StorageError> {
        let conn = self.connection.clone();
        spawn_blocking(move || {
            let mut conn = conn.get()
                .map_err(|e| domain_errors::StorageError::ConnectionFailed(e.to_string()))?;

            valid_urls_table
                .filter(short_url_column.eq(&short_url_value))
                .select(long_url_column)
                .first(&mut conn)
                .optional()
                .map_err(|err| domain_errors::StorageError::SelectionFailed(err.to_string()))
        }).await
        .map_err(|e| domain_errors::StorageError::TaskJoinError(e.to_string()))?
    }
}

impl UrlDatabase {
    // like a companion object
    pub fn new(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self { connection: conn }
    }
}
