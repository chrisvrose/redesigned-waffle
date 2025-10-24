use crate::{
    dto::userauth::OutUserDTO, errors::response::ResponseResult, misc::{auth::UserDetails, middleware::assert_role_auth, AppData}, models::{NewUserDTO, UserAuth}
};
use actix_web::{get, post, web::{self, Data, ReqData}, HttpResponse};

#[get("")]
pub async fn get_all(appstate: Data<AppData>,authdata:Option<ReqData<UserDetails>>) -> ResponseResult<web::Json<Vec<OutUserDTO>>> {
    let dbpool = &appstate.as_ref().pool;
    // TODO - add this back
    // let _ = assert_role_auth(authdata, Some(crate::misc::auth::UserType::Admin))?;
    let resp = UserAuth::get_all(dbpool).await?;
    Ok(web::Json(resp))
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
