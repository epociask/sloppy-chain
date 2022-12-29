use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};

use serde::{Deserialize, Serialize};

use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    ParseError(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

impl From<&String> for ErrorResponse {
    fn from(error_response: &String) -> Self {
        ErrorResponse {
            errors: vec![error_response.into()],
        }
    } 
}

impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse{ errors }
    }
}

// /// Automatically convert ApiErrors to external Response Errors
// ApiError -> ResponseError
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                HttpResponse::BadRequest().into()
            }
            ApiError::NotFound(message) => {
                HttpResponse::NotFound().into()
            }
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().into()
            }
            ApiError::Unauthorized(error) => {
                HttpResponse::Unauthorized().into()
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

// https://github.com/ddimaria/rust-actix-example/blob/d4cdc238c88c09ea748ef5f42243d96b682caf31/src/errors.rs