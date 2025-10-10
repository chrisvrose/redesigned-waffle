use actix_web::{get, Responder};


#[get("")]
pub async fn up() -> impl Responder {
    "Up!"
}
