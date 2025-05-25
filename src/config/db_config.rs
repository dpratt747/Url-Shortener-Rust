use std::env;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct DbConfig {
    host: String,
    port: String,
    user: String,
    password: String,
    database_name: String
}

impl DbConfig {
    pub(crate) fn from_env() -> Self {
        Self {
            host: env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("DB_PORT").unwrap_or_else(|_| "5432".into()),
            user: env::var("DB_USER").unwrap_or_else(|_| "postgres".into()),
            password: env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".into()),
            database_name: env::var("DB_NAME").unwrap_or_else(|_| "url-shortener-db".into()),
        }
    }

    pub(crate) fn url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.database_name)
    }
}