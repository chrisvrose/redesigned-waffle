use serde::Serialize;
use sqlx::PgPool;
pub mod auth;
pub struct AppData {
    pub pepper_secret: String,
    pub jwt_secret:String,
    pub pool: PgPool,
}


#[derive(Serialize)]
pub struct JsonResponse<T> {
    pub ok: bool,
    pub response: T,
}
impl<T> JsonResponse<T> {
    pub fn new(ok: bool, response: T) -> JsonResponse<T> {
        JsonResponse { ok, response }
    }
}
