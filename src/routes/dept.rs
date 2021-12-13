use actix_web::HttpResponse;
use actix_web::{web::Data, Responder,get};

use crate::types::AppData;
use crate::models::Dept;


#[get("")]
pub async fn get_all(appdata:Data<AppData>)->impl Responder{
    let db = & appdata.as_ref().pool;
    let data = Dept::get_all(db).await;
    match data{
        Ok(res)=>HttpResponse::Ok().json(res),
        _=>HttpResponse::InternalServerError().body("")
    }
}