mod config;
mod domain;
mod endpoints;
mod persistence;
mod services;

use crate::config::db_config::DbConfig;
use crate::endpoints::url_shortener_endpoints::{get_all, redirect_to_long_url, shorten, ApiDoc};
use crate::services::url_shortener_service::UrlShortenerService;

use crate::persistence::database::UrlDatabase;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_configuration = DbConfig::from_env();
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    let manager = ConnectionManager::<PgConnection>::new(db_configuration.url());
    let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
        .expect("Failed to create pool");

    let mut connection = pool.get().expect("Failed to get connection");
    connection
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|_| panic!("Error running migrations: {}", db_configuration.url()));

    let shared_pool = Arc::new(pool);
    let url_database = UrlDatabase::new(Arc::clone(&shared_pool));

    let service = Arc::new(Mutex::new(UrlShortenerService::new(Box::new(url_database))));

    // Default to localhost, use 0.0.0.0 if IN_DOCKER is set
    let addr = if env::var("IN_DOCKER").is_ok() {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    println!("Starting server on {}", addr);

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(Arc::clone(&service)))
            .service(web::scope("/v1").service(get_all).service(shorten))
            .service(redirect_to_long_url)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((addr, 8080))? // docker
    .run();

    // Spawn a task to log after the server is running
    tokio::spawn(async move {
        // Small delay to ensure the server is fully ready
        sleep(Duration::from_millis(500)).await;
        println!("The server has been started");
    });

    // Await the server (block until shutdown)
    server.await
}
