use actix_web::HttpResponse;
use actix_web::{
    post,
    web::{self, Data},
    Responder,
};
use serde_json::json;

use crate::misc::AppData;
use crate::models::{UserAuth, UserAuthCredsDTO};

#[post("")]
async fn login(appdata: Data<AppData>, logincreds: web::Json<UserAuthCredsDTO>) -> impl Responder {
    let dbpool = &(appdata.as_ref().pool);
    let jwt_key = &(appdata.as_ref().jwt_secret);
    let pepper_key = &appdata.as_ref().pepper_secret;
    let jwt = UserAuth::login(&logincreds, dbpool, pepper_key, jwt_key).await;

    jwt.map_or_else(
        |_x| HttpResponse::Forbidden().body(""),
        |jwt| {
            jwt.map_or_else(
                || HttpResponse::InternalServerError().body(""),
                |jwt| HttpResponse::Ok().json(json!({"ok":"true","response":jwt})),
            )
        },
    )
}
