use crate::{
    errors::response::ResponseResult, misc::AppData, models::{NewUserDTO, UserAuth}
};
use actix_web::{get, post, web::Data, HttpResponse};

#[get("")]
pub async fn get_all(appstate: Data<AppData>) -> ResponseResult<HttpResponse> {
    let dbpool = &appstate.as_ref().pool;
    let resp = UserAuth::get_all(dbpool).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[post("")]
pub async fn add_user(
    user: actix_web::web::Json<NewUserDTO>,
    appdata: Data<AppData>,
) -> ResponseResult<HttpResponse> {
    let dbpool = &appdata.as_ref().pool;
    let salt = &appdata.as_ref().salt_secret;
    let addedid = UserAuth::add_user(&user, dbpool, salt).await?;
    Ok(HttpResponse::Ok().json(addedid))
}
