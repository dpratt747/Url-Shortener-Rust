use crate::services::url_shortener_service::{UrlShortenerService, UrlShortenerServiceAlg};
use actix_web::web::Redirect;
use actix_web::Either;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Mutex;
use utoipa::OpenApi;
use crate::persistence::database::{LongUrl, ShortUrl};

#[derive(OpenApi)]
#[openapi(
    paths(get_all, shorten, redirect_to_long_url),
    components(schemas(ShortenUrlRequest))
)]
pub struct ApiDoc;
// https://chat.deepseek.com/a/chat/s/48e4c1e4-630e-4853-a228-66f5408de5a7

#[allow(non_snake_case)]
#[derive(Deserialize, utoipa::ToSchema)]
struct ShortenUrlRequest {
    longUrl: LongUrl,
}

#[utoipa::path(
    get,
    path = "/v1/all",
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
    path = "/v1/shorten",
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
        .store_long_url_and_get_short_url(info.longUrl.clone())
        .0;
    let full_endpoint = format!("http://localhost:8080/{short_url}");
    HttpResponse::Created().json(full_endpoint)
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
    path: web::Path<ShortUrl>,
) -> impl Responder {
    let long_url_opt = service
        .lock()
        .unwrap()
        .get_long_url_with_short(path.into_inner());

    match long_url_opt {
        Some(long_url) => Either::Right(Redirect::to(long_url.0).temporary()),
        None => Either::Left(HttpResponse::BadRequest().json("Url not found. Might have expired or it was not created")),
    }
}
