use actix_web::{get,post,web, HttpResponse, Responder};
use sqlx::{Pool,Postgres};
use crate::models::Subject;
use serde_json::value::Value;

#[get("")]
pub async fn get_all_subs(dbpool:web::Data<Pool<Postgres>>) -> impl Responder {
    // let mut x = HttpResponse::Ok();
    let vals = Subject::get_all(dbpool.get_ref()).await;
    match vals{
        Ok(v)=>HttpResponse::Ok().json(v),
        Err(_)=>HttpResponse::NotFound().json(serde_json::json!([]))
    }
    
}

#[post("")]
pub async fn add_sub(data:web::Json<Subject>,dbpool:web::Data<Pool<Postgres>>)->impl Responder{
    let data = data.into_inner();
    let response = Subject::insert(&data, &dbpool).await;
    match response{
        Ok(v)=>HttpResponse::Ok().json(serde_json::json!({"ok":v>0  })),
        Err(_)=>HttpResponse::BadRequest().json(serde_json::json!({"ok":false}))
    }
}
#[get("/{id}")]
pub async fn get_one(id:web::Path<String>,dbpool:web::Data<Pool<Postgres>>)->impl Responder{
    let str = id.into_inner();
    let vals = Subject::get_one(&str,&dbpool).await;
    match vals{
        Ok(Some(x))=>HttpResponse::Ok().json(x),
        Ok(None)=>HttpResponse::NotFound().json(Value::Null),
        _=>HttpResponse::InternalServerError().json(Value::Null)
    }
}
