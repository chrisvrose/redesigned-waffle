use actix_web::{
    get, post,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};

use crate::{misc::AppData, models::Subject};
use serde_json::value::Value;

#[get("all")]
pub async fn get_all_subs(appdata: Data<AppData>) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    // let mut x = HttpResponse::Ok();

    let vals = Subject::get_all(dbpool).await;
    if let Ok(vals) = vals {
        HttpResponse::Ok().json(vals)
    } else {
        HttpResponse::NotFound().json(serde_json::json!([]))
    }
}

#[get("")]
pub async fn get_user_subs(
    uid: Option<web::ReqData<i32>>,
    appdata: Data<AppData>,
) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    // let mut x = HttpResponse::Ok();

    match uid {
        Some(uid) => {
            let vals = Subject::get_for_user(&uid, dbpool).await;
            if let Ok(vals) = vals {
                HttpResponse::Ok().json(vals)
            } else {
                HttpResponse::NotFound().json(serde_json::json!([]))
            }
        }
        None => HttpResponse::NotFound().json(serde_json::json!([])),
    }
}

#[post("")]
pub async fn add_sub(data: web::Json<Vec<Subject>>, appdata: Data<AppData>) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    let data = data.into_inner();

    let response = Subject::insert(&data, dbpool).await;

    match response {
        Ok(v) => HttpResponse::Ok().json(serde_json::json!({ "ok": v })),
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({"ok":false})),
    }
}
#[get("/{id}")]
pub async fn get_one(id: web::Path<String>, appdata: Data<AppData>) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    let str = id.into_inner();
    let vals = Subject::get_one(&str, dbpool).await;
    match vals {
        Ok(Some(x)) => HttpResponse::Ok().json(x),
        Ok(None) => HttpResponse::NotFound().json(Value::Null),
        _ => HttpResponse::InternalServerError().json(Value::Null),
    }
}
