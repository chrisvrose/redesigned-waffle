use actix_web::web::ServiceConfig;

mod subject;
mod up;
mod userauth;

/// Function that adds the routes
pub fn init(serviceConfig: &mut ServiceConfig) {
    // add the up route
    serviceConfig.service(actix_web::web::scope("/up").service(up::up))
        // add the subject routes
        .service(
            actix_web::web::scope("/subject")
                .service(subject::get_all_subs)
                .service(subject::get_one)
                .service(subject::add_sub),
        )
        .service(
            actix_web::web::scope("/user")
                .service(userauth::get_all)
                .service(userauth::add_user),
        );
}
