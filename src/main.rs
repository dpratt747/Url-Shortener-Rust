mod endpoints;
mod persistence;
mod services;

use crate::endpoints::url_shortener_endpoints::{get_all, redirect_to_long_url, shorten, ApiDoc};
use crate::services::url_shortener_service::UrlShortenerService;
use actix_web::{web, App, HttpServer};
use persistence::database::InMemoryDatabase;
use std::collections::HashMap;
use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = InMemoryDatabase::new(HashMap::new());
    let service = UrlShortenerService::new(Box::new(db.clone()));
    let service_data = web::Data::new(Mutex::new(service));

    // Default to localhost, use 0.0.0.0 if IN_DOCKER is set
    let addr = if std::env::var("IN_DOCKER").is_ok() {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };


    HttpServer::new(move || {
        App::new()
            .app_data(service_data.clone())
            .service(
                web::scope("/v1")
                    .service(get_all)
                    .service(shorten)
            )
            .service(redirect_to_long_url)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((addr, 8080))?// docker
    .run()
    .await
}
