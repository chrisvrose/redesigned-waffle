use actix_web::ResponseError;
use actix_web::http::StatusCode;
use derive_more::Display;
use log::error;

#[derive(Debug, Display)]
pub enum ResponseErrors {
    #[display("Not authorised")]
    Unauthorized,
    #[display("Forbidden")]
    Forbidden,
    #[display("Something went wrong!")]
    InternalServerError,
    #[display("Bad Request: {_0}")]
    BadRequest(&'static str),
    #[display("Not found")]
    NotFound,
}

impl From<sqlx::Error> for ResponseErrors {
    fn from(value: sqlx::Error) -> Self {
        error!("SQLx error {}", value);
        match value{
            sqlx::Error::RowNotFound => ResponseErrors::NotFound,
            _ => ResponseErrors::InternalServerError,
        }
    }
}
impl From<argon2::Error> for ResponseErrors {
    fn from(value: argon2::Error) -> Self {
        error!("Argon2 error encountered {:?}",value);
        match value {
            argon2::Error::PwdTooShort | argon2::Error::PwdTooLong => {
                ResponseErrors::BadRequest("Invalid password")
            }
            argon2::Error::DecodingFail => ResponseErrors::Forbidden,
            _ => ResponseErrors::InternalServerError,
        }
    }
}
impl From<jsonwebtoken::errors::Error> for ResponseErrors{
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        ResponseErrors::InternalServerError
    }
}

pub type ResponseResult<T> = Result<T, ResponseErrors>;

impl ResponseError for ResponseErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            ResponseErrors::Forbidden => StatusCode::FORBIDDEN,
            ResponseErrors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseErrors::BadRequest(_) => StatusCode::BAD_REQUEST,
            ResponseErrors::NotFound => StatusCode::NOT_FOUND,
            ResponseErrors::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
