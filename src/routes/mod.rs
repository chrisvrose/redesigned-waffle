use actix_web::web::ServiceConfig;

mod subject;
mod up;
mod userauth;
mod dept;
mod auth;

/// Function that adds the routes
pub fn init(service_config: &mut ServiceConfig) {
    // add the up route
    service_config
        .service(actix_web::web::scope("/up").service(up::up))
        // add the subject routes
        .service(
            actix_web::web::scope("/subject")
                .service(subject::get_all_subs)
                .service(subject::get_user_subs)
                .service(subject::get_one)
                .service(subject::add_subs),
        )
        .service(
            actix_web::web::scope("/user")
                .service(userauth::get_all)
                .service(userauth::add_user),
        )
        .service(actix_web::web::scope("/auth").service(auth::login_student))

        .service(
            actix_web::web::scope("/dept").service(dept::get_all)
        );
}
