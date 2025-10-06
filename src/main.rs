use actix_web::{dev::Service, middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;

use log::{debug, trace};
use misc::{middleware::jwt_authentication, AppData};
use sqlx::{self, PgPool};

use crate::misc::env::AppConfigVariables;

mod dto;
mod misc;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();
    trace!("App started");
    // prepare the required data and inject it
    let AppConfigVariables {
        database_url,
        jwt_secret,
        salt,
        address,
        port,
    } = AppConfigVariables::from_env();
    // prepare DB connection pool

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect");
    debug!("Got DB Connection, trying to start server");

    HttpServer::new(move || {
        let jwt_secret_for_middleware = jwt_secret.clone();
        App::new()
            // just inspect the authorization header and use it
            .wrap_fn(move |req, srv| {
                jwt_authentication(&req, &jwt_secret_for_middleware);
                srv.call(req)
            })
            // insert app data
            .app_data(Data::new(AppData::new(
                salt.clone(),
                jwt_secret.clone(),
                pool.clone(),
            )))
            // Create Logger
            .wrap(Logger::default())
            // add a set of routes
            .configure(crate::routes::init)
    })
    .bind((address, port))?
    .run()
    .await
}
