use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Unauthorized(&'static str),
    Forbidden(&'static str),
    NotFound(String),
    BadRequest(String),
    Internal(String),
    Database(mongodb::error::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized(msg)  => write!(f, "{}", msg),
            Self::Forbidden(msg)     => write!(f, "{}", msg),
            Self::NotFound(msg)      => write!(f, "{}", msg),
            Self::BadRequest(msg)    => write!(f, "{}", msg),
            Self::Internal(msg)      => write!(f, "{}", msg),
            Self::Database(e)        => write!(f, "database error: {}", e),
        }
    }
}

impl ResponseError for AppError {

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Internal(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            Self::Unauthorized(msg)  => (401, msg.to_string()),
            Self::Forbidden(msg)     => (403, msg.to_string()),
            Self::NotFound(msg)      => (404, msg.clone()),
            Self::BadRequest(msg)    => (400, msg.clone()),
            Self::Internal(msg)      => (500, msg.clone()),
            Self::Database(_)        => (500, "database error".into()),
        };
        HttpResponse::build(
            actix_web::http::StatusCode::from_u16(status).unwrap()
        )
        .json(json!({ "error": message, "statusCode": status }))
    }
}

// So you can do mongo_result.map_err(AppError::Database)?
impl From<mongodb::error::Error> for AppError {
    fn from(e: mongodb::error::Error) -> Self {
        Self::Database(e)
    }
}
