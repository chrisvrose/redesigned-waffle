use actix_web::{get, HttpResponse, Responder};
use crate::types::JsonResponse;


#[get("/")]
pub async fn up() -> impl Responder {
    HttpResponse::Ok().json(JsonResponse::new(true,true))
}
