mod endpoints;
mod persistence;
mod services;

use crate::endpoints::url_shortener_endpoints::{get_all, redirect_to_long_url, shorten, ApiDoc};
use crate::services::url_shortener_service::UrlShortenerService;
use actix_web::{web, App, HttpServer};
use persistence::database::InMemoryDatabase;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use tokio_cron_scheduler::{Job, JobScheduler};
use chrono::Utc;
use crate::persistence::database::{DatabaseAlg, LongUrl, ShortUrl};

async fn start_scheduler(db: Box<dyn DatabaseAlg>) -> Result<(), Box<dyn std::error::Error>> {
// async fn start_scheduler(db: Box<dyn DatabaseAlg>) -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;
    
    let every_thirty_mins = "0 */30 * * * *";
    let every_ten_seconds = "1/10 * * * * *";
    let every_second = "1/1 * * * * *";

    // let all_stored_map: HashMap<LongUrl, ShortUrl> = db.get_all();
        // .iter()
        // .map(|(long_url, (short_url, _))| (long_url.0.clone(), short_url.0.clone()))
        // .collect();

    // Add a job that runs every 10 seconds
    sched.add(
        Job::new_async(every_second, move |_uuid, _l| {
            let get_all_stored = db.get_all();
            // let new_db = db.get_all();
            Box::pin(async move {
                println!("Scheduled job running at {:?}", Utc::now());
                println!("all stored urls: {:?}", get_all_stored);
                // Ok(())
            })
        })?
    ).await?;

    sched.start().await?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(Mutex::new(InMemoryDatabase::new(HashMap::new())));
    // db.store(LongUrl("some long url".to_string()), ShortUrl("Some short url".to_string()));
    
    let service = UrlShortenerService::new(Box::new(db.clone()));
    let service_data = web::Data::new(Mutex::new(service));

    // Start the scheduler in a separate task
    tokio::spawn(async move {
        if let Err(e) = start_scheduler(Box::new(db.clone())).await {
            eprintln!("Scheduler error: {}", e);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(service_data.clone())
            .service(get_all)
            .service(shorten)
            .service(redirect_to_long_url)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
