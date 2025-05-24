mod config;
mod domain;
mod endpoints;
mod persistence;
mod schema;
mod services;

use crate::config::db_config::DbConfig;
use crate::endpoints::url_shortener_endpoints::{get_all, redirect_to_long_url, shorten, ApiDoc};
use crate::services::url_shortener_service::UrlShortenerService;

use crate::persistence::database::{DatabaseAlg, UrlDatabase};
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use persistence::database::InMemoryDatabase;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_configuration = DbConfig::from_env();

    let db = InMemoryDatabase::new(HashMap::new());
    let service = UrlShortenerService::new(Box::new(db.clone()));
    let service_data = web::Data::new(Mutex::new(service));

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    // test db here:

    let mut conn = PgConnection::establish(&db_configuration.url())
        .expect(&format!("Error connecting to {}", db_configuration.url()));
    

    conn.run_pending_migrations(MIGRATIONS).expect(&format!(
        "Error running migrations: {}",
        db_configuration.url()
    ));
    
    let shared_conn = Arc::new(Mutex::new(conn));
    let url_database = UrlDatabase::new(Arc::clone(&shared_conn));
    
    println!("Testing database connection... {:?}", url_database.get_all());
    
    // Default to localhost, use 0.0.0.0 if IN_DOCKER is set
    let addr = if env::var("IN_DOCKER").is_ok() {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    println!("Starting server on {}", addr);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(service_data.clone())
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
