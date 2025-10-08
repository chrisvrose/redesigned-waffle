use actix_web::HttpResponse;
use actix_web::{
    post,
    web::{self, Data},
    Responder,
};
use serde_json::json;

use crate::dto::UserAuthCredsDTO;
use crate::misc::AppData;
use crate::models::{UserAuth};

#[post("")]
async fn login(appdata: Data<AppData>, logincreds: web::Json<UserAuthCredsDTO>) -> impl Responder {
    let dbpool = &(appdata.as_ref().pool);
    let jwt_key = &(appdata.as_ref().jwt_secret);

    let jwt_option_result = UserAuth::login_student(&logincreds, dbpool, jwt_key).await;

    jwt_option_result.map_or_else(
        |_x| HttpResponse::Forbidden().body(""),
        |jwt_option| {
            jwt_option.map_or_else(
                || HttpResponse::InternalServerError().body(""),
                |jwt| HttpResponse::Ok().json(jwt),
            )
        },
    )
}
