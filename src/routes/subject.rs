use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use log::error;
use sqlx::Error;

use crate::{
    misc::{
        auth::{UserDetails, UserType},
        AppData,
    },
    models::Subject,
};
use serde_json::value::Value;

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
) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    match user_details_opt {
        Some(usertype) => {
            let UserDetails { uid, user_type } = usertype.into_inner();
            let subjects_list_results = match user_type {
                UserType::Student => {
                    Subject::get_for_user(&uid, dbpool).await
                }
                UserType::Admin => {
                    Subject::get_all(dbpool).await
                }
            };
            if let Ok(vals) = subjects_list_results {
                HttpResponse::Ok().json(vals)
            } else {
                HttpResponse::NotFound()
                    .json(serde_json::json!({"message":"Could not get records"}))
            }
        }
        None => HttpResponse::Forbidden()
            .json(serde_json::json!({"message":"Not a valid user"})),
    }
}

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
