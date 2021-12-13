use actix_web::{get,post,web::{self, Data}, HttpResponse, Responder};

use crate::{models::Subject, types::AppData};
use serde_json::value::Value;

#[get("")]
pub async fn get_all_subs(appdata:Data<AppData>) -> impl Responder {
    let dbpool = &appdata.as_ref().pool;

    // let mut x = HttpResponse::Ok();
    let vals = Subject::get_all(dbpool).await;
    match vals{
        Ok(v)=>HttpResponse::Ok().json(v),
        Err(_)=>HttpResponse::NotFound().json(serde_json::json!([]))
    }
    
}

#[post("")]
pub async fn add_sub(data:web::Json<Subject>,appdata:Data<AppData>)->impl Responder{
    let dbpool = &appdata.as_ref().pool;

    let data = data.into_inner();
    let response = Subject::insert(&data, dbpool).await;
    match response{
        Ok(v)=>HttpResponse::Ok().json(serde_json::json!({"ok":v>0  })),
        Err(_)=>HttpResponse::BadRequest().json(serde_json::json!({"ok":false}))
    }
}
#[get("/{id}")]
pub async fn get_one(id:web::Path<String>,appdata:Data<AppData>)->impl Responder{
    let dbpool = &appdata.as_ref().pool;

    let str = id.into_inner();
    let vals = Subject::get_one(&str,dbpool).await;
    match vals{
        Ok(Some(x))=>HttpResponse::Ok().json(x),
        Ok(None)=>HttpResponse::NotFound().json(Value::Null),
        _=>HttpResponse::InternalServerError().json(Value::Null)
    }
}
