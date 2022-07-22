use actix_web::{dev::Service, middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;

use log::{debug, trace};
use misc::{middleware::do_auth_insert, AppData};
use sqlx::{self, PgPool};
// load modules
mod misc;
mod models;
mod routes;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();
    trace!("App started");
    // prepare the required data and inject it
    let database_url = std::env::var("DATABASE_URL").expect("Nothing");
    let salt = std::env::var("SALTEDSECRET").expect("No Salted secret");
    let jwt_secret = std::env::var("JWTSECRET").expect("No JWT secret");
    // prepare DB connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect");

    debug!("Got DB Connection, trying to start server");

    HttpServer::new(move || {
        let jwt_secret = jwt_secret.clone();

        // this is a copy made for the middleware to use -> since this function will die after creation, this copy will be used by the middleware later
        let jwt_secret_for_middleware = jwt_secret.clone();
        App::new()
            // just inspect the authorization header and use it
            .wrap_fn(move |req, srv| {
                do_auth_insert(&req, &jwt_secret_for_middleware);
                srv.call(req)
            })
            // insert app data
            .app_data(Data::new(AppData {
                pool: pool.clone(),
                salt_secret: salt.clone(),
                jwt_secret,
            }))
            // Create Logger
            .wrap(Logger::default())
            // add a set of routes
            .configure(crate::routes::init)
    })
    .bind(format!("{}:{}", ADDRESS, PORT))?
    .run()
    .await
}
