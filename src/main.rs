use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use misc::AppData;
use sqlx::{self, PgPool};
use dotenv::dotenv;
// load modules
mod middleware;
mod misc;
mod models;
mod routes;

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Nothing");
    let salt = std::env::var("SALTEDSECRET").expect("No Salted secret");
    let jwt_secret = std::env::var("JWTSECRET").expect("No JWT secret");
    // pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppData {
                pool: pool.clone(),
                pepper_secret: salt.clone(),
                jwt_secret: jwt_secret.clone()
            }))
            // .app_data(Data::new(salt.clone()))
            .wrap(Logger::default())
            // add a set of routes
            .configure(crate::routes::init)
    })
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}
