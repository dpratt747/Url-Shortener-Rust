use crate::services::url_shortener_service::{UrlShortenerService, UrlShortenerServiceAlg};
use actix_web::web::Redirect;
use actix_web::Either;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Mutex;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_all, shorten, redirect_to_long_url),
    components(schemas(ShortenUrlRequest))
)]
pub struct ApiDoc;

#[allow(non_snake_case)]
#[derive(Deserialize, utoipa::ToSchema)]
struct ShortenUrlRequest {
    longUrl: String,
}

#[utoipa::path(
    get,
    path = "/all",
    responses(
        (status = 200, description = "Success response")
    )
)]
#[get("/all")]
async fn get_all(service: web::Data<Mutex<UrlShortenerService>>) -> impl Responder {
    let urls = service.lock().unwrap().get_all();
    HttpResponse::Ok().json(urls)
}

#[utoipa::path(
    post,
    path = "/shorten",
    request_body = ShortenUrlRequest,
    responses(
        (status = 200, description = "Success response")
    )
)]
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

#[utoipa::path(
    get,
    path = "/{short_url_path}",
    params(
        ("short_url_path" = String, Path, description = "Short URL path")
    ),
    responses(
        (status = 302, description = "Redirect to long URL"),
        (status = 400, description = "URL not found")
    )
)]
#[get("/{short_url_path}")]
async fn redirect_to_long_url(
    service: web::Data<Mutex<UrlShortenerService>>,
    path: web::Path<String>,
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
