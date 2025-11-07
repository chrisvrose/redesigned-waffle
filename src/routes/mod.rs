use actix_web::{get, web::ServiceConfig, Responder};

mod auth;
mod book;
mod dept;
mod course;
mod up;
mod userauth;

#[get("/")]
pub async fn hello_world() -> impl Responder{
    "redesigned-waffle"
}


/// Function that adds the routes
pub fn init(service_config: &mut ServiceConfig) {
    // add the up route
    service_config
        // .service(hello_world)
        .service(actix_web::web::scope("/up").service(up::up))
        // add the subject routes
        .service(
            actix_web::web::scope("/course")
                .service(course::get_all_subs)
                .service(course::get_user_subs)
                .service(course::get_one)
                .service(course::add_subs),
        )
        .service(
            actix_web::web::scope("/user")
                .service(userauth::get_all)
                .service(userauth::add_user)
                .service(userauth::get_self),
        )
        .service(actix_web::web::scope("/auth").service(auth::login))
        .service(actix_web::web::scope("/dept").service(dept::get_all))
        .service(
            actix_web::web::scope("/book")
                .service(book::get_user)
                .service(book::make_booking)
                .service(book::get_calc)
        )
        .service(actix_files::Files::new("/","./static").index_file("index.html"));
}
