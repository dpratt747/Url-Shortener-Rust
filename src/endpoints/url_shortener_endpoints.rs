use crate::domain::requests::objects::ShortenUrlRequest;
use crate::domain::responses::objects::UrlPairResponse;
use crate::domain::types::objects;
use crate::services::url_shortener_service::{UrlShortenerService, UrlShortenerServiceAlg};
use actix_web::web::Redirect;
use actix_web::Either;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Mutex;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_all, shorten, redirect_to_long_url),
    components(schemas(ShortenUrlRequest))
)]
pub struct ApiDoc;
// https://chat.deepseek.com/a/chat/s/48e4c1e4-630e-4853-a228-66f5408de5a7

#[utoipa::path(
    get,
    path = "/v1/all",
    responses(
        (status = 200, description = "Success response")
    )
)]
#[get("/all")]
async fn get_all(service: web::Data<Mutex<UrlShortenerService>>) -> impl Responder {
    match service.lock().unwrap().get_all().await {
        Ok(url_response) => {
            let url_response_objects: Vec<UrlPairResponse> = url_response
                .into_iter()
                .map(|url| {
                    let response_object: UrlPairResponse = url.into();
                    response_object
                })
                .collect();
            HttpResponse::Ok().json(url_response_objects)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
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
    let response = service
        .lock()
        .unwrap()
        .store_long_url_and_get_short_url(info.longUrl.clone()).await;

    match response {
        Ok(url) => {
            let full_endpoint = format!("http://localhost:8080/{url}");
            HttpResponse::Created().json(full_endpoint)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
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
    path: web::Path<objects::ShortUrl>,
) -> impl Responder {
    let result = service
        .lock()
        .unwrap()
        .get_long_url_with_short(path.into_inner())
        .await;

    match result {
        Ok(long_url_opt) => match long_url_opt {
            Some(long_url) => Either::Right(Redirect::to(long_url.0).temporary()),
            None => Either::Left(
                HttpResponse::BadRequest()
                    .json("Url not found. Might have expired or it was not created"),
            ),
        },
        Err(e) => Either::Left(HttpResponse::InternalServerError().body(e.to_string())),
    }
}
