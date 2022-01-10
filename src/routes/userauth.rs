use crate::{models::{NewUserDTO, UserAuth}, misc::AppData};
use actix_web::{get, post, web::Data, HttpResponse, Responder};


#[get("")]
pub async fn get_all(appstate: Data<AppData>) -> impl Responder {
    let dbpool = &appstate.as_ref().pool;
    let resp = UserAuth::get_all(dbpool).await;
    match resp {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[post("")]
pub async fn add_user(
    user: actix_web::web::Json<NewUserDTO>,
    appdata: Data<AppData>,
    
) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;
    let salt = &appdata.as_ref().pepper_secret;
    let resp = UserAuth::add_user(&user, dbpool, salt).await;
    match resp {
        Ok(addedid) => HttpResponse::Ok().json(addedid),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}
