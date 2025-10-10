use actix_web::HttpResponse;
use actix_web::{web::Data, get};

use crate::errors::response::ResponseErrors;
use crate::misc::AppData;
use crate::models::Dept;


#[get("")]
pub async fn get_all(appdata:Data<AppData>)->Result<HttpResponse,ResponseErrors>{
    let db = & appdata.as_ref().pool;
    let data = Dept::get_all(db).await?;
    Ok(HttpResponse::Ok().json(data))
}
