use crate::services::url_shortener_service::{UrlShortenerService, UrlShortenerServiceAlg};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Mutex;
use actix_web::web::Redirect;
use actix_web::Either;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct ShortenUrlRequest {
    longUrl: String,
}

#[get("/all")]
async fn get_all(service: web::Data<Mutex<UrlShortenerService>>) -> impl Responder {
    let urls = service.lock().unwrap().get_all();
    HttpResponse::Ok().json(urls)
}

#[post("/shorten")]
async fn shorten(
    service: web::Data<Mutex<UrlShortenerService>>,
    info: web::Json<ShortenUrlRequest>,
) -> impl Responder {
    let short_url = service
        .lock()
        .unwrap()
        .store_long_url_and_get_short_url(info.longUrl.clone());
    HttpResponse::Ok().json(short_url)
}

#[get("/{short_url_path}")]
async fn redirect_to_long_url(
    service: web::Data<Mutex<UrlShortenerService>>,
    path: web::Path<String>
) -> impl Responder {
    let long_url_opt = service
        .lock()
        .unwrap()
        .get_long_url_with_short(path.to_string());
    
    match long_url_opt {
        Some(value) => Either::Left(Redirect::to(value).temporary()),
        None => Either::Right(HttpResponse::BadRequest().body("Url not found")),
    }
}
