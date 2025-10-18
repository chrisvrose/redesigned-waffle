use actix_web::http::StatusCode;
use actix_web::ResponseError;
use derive_more::Display;

#[derive(Debug,Display)]
pub enum ResponseErrors {
    #[display("Access denied")]
    AccessDenied,
    #[display("Something went wrong!")]
    InternalServerError,
    #[display("Huh? {_0}")]
    InternalServerErrorWithMessage(String),
    #[display("Bad Request: {_0}")]
    BadRequest(String),
    #[display("Not found")]
    NotFound
}

impl From<sqlx::Error> for ResponseErrors{
    fn from(_value: sqlx::Error) -> Self {
        ResponseErrors::InternalServerError
    }
}

pub type ResponseResult<T> = Result<T,ResponseErrors>;

impl ResponseError for ResponseErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            ResponseErrors::AccessDenied => StatusCode::FORBIDDEN,
            ResponseErrors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseErrors::InternalServerErrorWithMessage(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseErrors::BadRequest(_) => StatusCode::BAD_REQUEST,
            ResponseErrors::NotFound => StatusCode::NOT_FOUND
        }
    }
}
