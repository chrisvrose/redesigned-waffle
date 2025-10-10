use sqlx::PgPool;
pub mod middleware;
pub mod auth;
pub mod argon2_config;
pub mod env;



pub struct AppData {
    pub salt_secret: String,
    pub jwt_secret:String,
    pub pool: PgPool,
}
impl AppData{
    pub fn new(salt_secret: String, jwt_secret: String, pool: PgPool)->Self{
        AppData{
            salt_secret,jwt_secret,pool
        }
    }
}
