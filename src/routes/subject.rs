use actix_web::{
    get, post, web::{self, Data, Json}, HttpResponse, Responder
};
use log::error;
use sqlx::Error;

use crate::{
    errors::response::ResponseErrors,
    misc::{
        AppData,
        auth::{UserDetails, UserType},
        middleware::assert_role_auth,
    },
    models::Subject,
};
use serde_json::value::Value;

// TODO fix this
#[get("all")]
pub async fn get_all_subs(appdata: Data<AppData>) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    // let mut x = HttpResponse::Ok();

    let vals = Subject::get_all(dbpool).await;
    if let Ok(vals) = vals {
        HttpResponse::Ok().json(vals)
    } else {
        HttpResponse::NotFound()
            .json(serde_json::json!({"ok":false,"reason":"Could not get records"}))
    }
}


/// show possible subs for user or all for teachers
#[get("")]
pub async fn get_user_subs(
    user_details_opt: Option<web::ReqData<UserDetails>>,
    appdata: Data<AppData>,
) -> Result<web::Json<Vec<Subject>>, ResponseErrors> {
    let dbpool = &appdata.as_ref().pool;
    let UserDetails { uid, user_type } = assert_role_auth(user_details_opt, None)?;

    // let = user_details;
    let subjects_list_results = match user_type {
        UserType::Student => Subject::get_for_user(&uid, dbpool).await,
        UserType::Admin => Subject::get_all(dbpool).await,
    }?;
    Ok(web::Json(subjects_list_results))
}

// TODO fix this
#[post("")]
pub async fn add_subs(data: web::Json<Vec<Subject>>, appdata: Data<AppData>) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    let data = data.into_inner();

    let response = Subject::insert_all(&data, dbpool).await;

    match response {
        Ok(v) => HttpResponse::Ok().json(serde_json::json!({ "ok": v })),
        Err(Error::Database(err)) => HttpResponse::BadRequest()
            .json(serde_json::json!({"ok":false,"reason":err.to_string()})),
        Err(err) => {
            error!("Error adding subs {}", err.to_string());
            HttpResponse::BadRequest().json(serde_json::json!({
                "ok":false,"reason":"Error adding subjects"
            }))
        }
    }
}

// TODO fix
#[get("/{id}")]
pub async fn get_one(id: web::Path<String>, appdata: Data<AppData>) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    let str = id.into_inner();
    log::trace!("Attempting to fetch course code {}", str);
    let vals = Subject::get_one(&str, dbpool).await;
    match vals {
        Ok(Some(x)) => HttpResponse::Ok().json(x),
        Ok(None) => HttpResponse::NotFound().json(Value::Null),
        _ => HttpResponse::InternalServerError().json(Value::Null),
    }
}
