use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{self, PgPool};
use types::AppData;
// use uuid::Uuid;
// load modules
mod routes;
mod models;
mod types;

const PORT:u16=8080;
// #[derive(Serialize, Deserialize, Debug)]
// struct UserAuth {
//     id: i32,
//     name: String,
//     pwd: String,
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Nothing");
    let salt = std::env::var("SALTEDSECRET").expect("No Salted secret");
    // pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppData{pool: pool.clone(),salt:salt.clone()}))
            // .app_data(Data::new(salt.clone()))
            .wrap(Logger::default())
            // add a set of routes
            .configure(crate::routes::init)
    })
    .bind(format!("127.0.0.1:{}",PORT))?
    .run()
    .await
}
