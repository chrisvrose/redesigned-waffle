use actix_web::{dev::Service, middleware::Logger, web::Data, App, HttpMessage, HttpServer};
use dotenv::dotenv;
use futures::future::FutureExt;
use misc::{auth::validate_jwt, AppData};
use models::UserAuth;
use sqlx::{self, PgPool};
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
        let jwt_secret = jwt_secret.clone();

        // this is a copy made for the middleware to use -> since this function will die after creation, this copy will be used by the middleware later
        let jwt_secret_for_middleware = jwt_secret.clone();
        App::new()
            .wrap_fn(move |req, srv| {
                let header = req.headers().get("Authorization");
                if let Some(header) = header {
                    if let Ok(x) = header.to_str() {
                        let res = x.trim().split(' ').nth(1);
                        if let Some(res) = res {
                            let res: &String = &res.into();

                            if let Ok(user) = validate_jwt(&jwt_secret_for_middleware, res) {
                                let mut exts = req.extensions_mut();
                                exts.insert(user.uid);
                            }
                        }
                    }
                }
                srv.call(req).map(|res| res)
            })
            .app_data(Data::new(AppData {
                pool: pool.clone(),
                pepper_secret: salt.clone(),
                jwt_secret: jwt_secret,
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
