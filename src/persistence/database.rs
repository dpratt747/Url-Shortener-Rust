use crate::domain::errors::domain_errors;
use crate::domain::persistence::models;
use crate::domain::types::url;
use crate::schema::urls::dsl::urls as url_table;
use crate::schema::urls::{long_url as long_url_column, short_url as short_url_column};

use crate::domain::persistence::models::UrlPair;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;

pub trait DatabaseAlg: Send + Sync {
    fn store(
        &self,
        long_url_value: url::LongUrl,
        short_url_value: url::ShortUrl,
    ) -> Result<(), domain_errors::StorageError>;
    fn get_all(&self) -> Result<Vec<UrlPair>, domain_errors::StorageError>;
    // fn get_all(&self) -> Vec<(url::LongUrl, url::ShortUrl)>;
    fn get_long_url_with_short_url(
        &self,
        short_url: url::ShortUrl,
    ) -> Result<Option<url::LongUrl>, domain_errors::StorageError>;
}

pub struct UrlDatabase {
    connection: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl DatabaseAlg for UrlDatabase {
    fn store(
        &self,
        long_url_value: url::LongUrl,
        short_url_value: url::ShortUrl,
    ) -> Result<(), domain_errors::StorageError> {
        // Get connection with proper error handling
        let mut conn = self
            .connection
            .get()
            .map_err(|e| domain_errors::StorageError::ConnectionFailed(e.to_string()))?;

        let insert_url = models::InsertUrls {
            long_url: long_url_value,
            short_url: short_url_value,
        };

        // Execute query with proper error handling
        diesel::insert_into(url_table)
            .values(&insert_url)
            .execute(&mut conn)
            .map_err(|err| domain_errors::StorageError::from(err))?;

        Ok(())
    }

    fn get_all(&self) -> Result<Vec<UrlPair>, domain_errors::StorageError> {
        let mut conn = self
            .connection
            .get()
            .map_err(|e| domain_errors::StorageError::ConnectionFailed(e.to_string()))?;

        let results = url_table
            .select((long_url_column, short_url_column))
            .load::<UrlPair>(&mut conn)
            .map_err(|err| domain_errors::StorageError::SelectionFailed(err.to_string()))?;

        Ok(results)
    }

    fn get_long_url_with_short_url(
        &self,
        short_url_value: url::ShortUrl,
    ) -> Result<Option<url::LongUrl>, domain_errors::StorageError> {
        let mut conn = self
            .connection
            .get()
            .map_err(|e| domain_errors::StorageError::ConnectionFailed(e.to_string()))?;

        let res = url_table
            .filter(short_url_column.eq(&short_url_value))
            .select(long_url_column)
            .first(&mut conn)
            .optional()
            .map_err(|err| domain_errors::StorageError::SelectionFailed(err.to_string()))?;

        Ok(res)
    }
}

impl UrlDatabase {
    // like a companion object
    pub fn new(conn: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self { connection: conn }
    }
}
