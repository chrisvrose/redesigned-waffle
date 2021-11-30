use actix_web::{get,post,Responder,web::{Data},HttpResponse};
use sqlx::PgPool;
use crate::models::UserAuth;


#[get("/")]
pub async fn get_all(dbpool:Data<PgPool>)->impl Responder{
    let resp = UserAuth::get_all(dbpool.as_ref()).await;
    match resp{
        Ok(users)=>HttpResponse::Ok().json(users),
        Err(x)=>HttpResponse::InternalServerError().body("")
    }
}