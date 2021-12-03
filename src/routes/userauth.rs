use crate::models::{NewUserDTO, UserAuth};
use actix_web::{get, post, web::Data, HttpResponse, Responder};
use sqlx::PgPool;

#[get("")]
pub async fn get_all(dbpool: Data<PgPool>) -> impl Responder {
    let resp = UserAuth::get_all(dbpool.as_ref()).await;
    match resp {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[post("")]
pub async fn add_user(
    user: actix_web::web::Json<NewUserDTO>,
    dbpool: Data<PgPool>,
    salt: Data<String>,
) -> impl Responder {
    let resp = UserAuth::add_user(&user, dbpool.as_ref(), salt.as_ref()).await;
    match resp {
        Ok(addedid) => HttpResponse::Ok().json(addedid),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}
