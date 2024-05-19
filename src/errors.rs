use std::error::Error;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
    IoError(std::io::Error),
    DieselError(diesel::result::Error),
    SerdeJsonError(serde_json::Error),
    ActixWebError(actix_web::Error),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(e) => write!(f, "Bad Request: {}", e),
            ApiError::NotFound(e) => write!(f, "Not Found: {}", e),
            ApiError::InternalServerError(e) => write!(f, "Internal Server Error: {}", e),
            ApiError::IoError(e) => write!(f, "IO Error: {}", e),
            ApiError::DieselError(e) => write!(f, "Diesel Error: {}", e),
            ApiError::SerdeJsonError(e) => write!(f, "Serde JSON Error: {}", e),
            ApiError::ActixWebError(e) => write!(f, "Actix Web Error: {}", e),
        }
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ApiError::IoError(e) => Some(e),
            ApiError::DieselError(e) => Some(e),
            ApiError::SerdeJsonError(e) => Some(e),
            ApiError::ActixWebError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        ApiError::IoError(e)
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(e: diesel::result::Error) -> Self {
        ApiError::DieselError(e)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::SerdeJsonError(e)
    }
}

impl From<actix_web::Error> for ApiError {
    fn from(e: actix_web::Error) -> Self {
        ApiError::ActixWebError(e)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(_) => HttpResponse::BadRequest().finish(),
            ApiError::NotFound(_) => HttpResponse::NotFound().finish(),
            ApiError::InternalServerError(_) => HttpResponse::InternalServerError().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
