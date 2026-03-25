use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use std::fmt;

use crate::common::response::ApiResponse;

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
            Self::Unauthorized(msg) => write!(f, "{}", msg),
            Self::Forbidden(msg) => write!(f, "{}", msg),
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::BadRequest(msg) => write!(f, "{}", msg),
            Self::Internal(msg) => write!(f, "{}", msg),
            Self::Database(e) => write!(f, "database error: {}", e),
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
        
        let status = self.status_code();

        let body = ApiResponse::<()>::message(status, self.to_string());
        
        HttpResponse::build(status).json(body)
    }
}

// So you can do mongo_result.map_err(AppError::Database)?
impl From<mongodb::error::Error> for AppError {
    fn from(e: mongodb::error::Error) -> Self {
        Self::Database(e)
    }
}
