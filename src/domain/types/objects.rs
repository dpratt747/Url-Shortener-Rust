use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    utoipa::ToSchema,
    AsExpression,
    FromSqlRow,
)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct LongUrl(pub(crate) String);

// Implement ToSql for database writes
impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for LongUrl
where
    DB: diesel::backend::Backend,
    String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        self.0.to_sql(out)
    }
}

// Implement FromSql for database reads
impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for LongUrl
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        String::from_sql(bytes).map(LongUrl)
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    utoipa::ToSchema,
    AsExpression,
    FromSqlRow,
)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct ShortUrl(pub(crate) String);

impl fmt::Display for ShortUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0) // Assuming ShortUrl wraps a String or displayable type
    }
}

// Implement ToSql for database writes
impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for ShortUrl
where
    DB: diesel::backend::Backend,
    String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        self.0.to_sql(out)
    }
}

// Implement FromSql for database reads
impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for ShortUrl
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        String::from_sql(bytes).map(ShortUrl)
    }
}
