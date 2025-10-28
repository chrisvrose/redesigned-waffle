use actix_web::HttpResponse;
use actix_web::{
    Responder, post,
    web::{self, Data},
};
use log::info;

use crate::dto::UserAuthCredsDTO;
use crate::misc::AppData;
use crate::models::UserAuth;

#[post("")]
async fn login(appdata: Data<AppData>, logincreds: web::Json<UserAuthCredsDTO>) -> impl Responder {
    let dbpool = &(appdata.as_ref().pool);
    let jwt_key = &(appdata.as_ref().jwt_secret);

    let jwt_option_result = UserAuth::login(&logincreds, dbpool, jwt_key).await;
    info!("Login {} {:?}", logincreds.email, jwt_option_result);
    jwt_option_result.map_or_else(
        |_x| HttpResponse::Forbidden().body(""),
        |jwt| HttpResponse::Ok().body(jwt),
    )
}
