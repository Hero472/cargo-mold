use serde::Serialize;
use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::StatusCode};

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize = ()> {
    #[serde(skip)]
    pub status_code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    // ---- 2xx Success (with data) ----
    pub fn ok(message: impl Into<String>, data: T) -> Self {
        Self { status_code: 200, message: message.into(), data: Some(data) }
    }

    pub fn created(message: impl Into<String>, data: T) -> Self {
        Self { status_code: 201, message: message.into(), data: Some(data) }
    }

    pub fn accepted(message: impl Into<String>, data: T) -> Self {
        Self { status_code: 202, message: message.into(), data: Some(data) }
    }

    // ---- 2xx Success (no data) ----
    pub fn success(message: impl Into<String>) -> Self {
        Self { status_code: 200, message: message.into(), data: None }
    }

    pub fn no_content(message: impl Into<String>) -> Self {
        Self { status_code: 204, message: message.into(), data: None }
    }

    // ---- 4xx Client Errors (no data) ----
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self { status_code: 400, message: message.into(), data: None }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self { status_code: 401, message: message.into(), data: None }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self { status_code: 403, message: message.into(), data: None }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self { status_code: 404, message: message.into(), data: None }
    }

    pub fn method_not_allowed(message: impl Into<String>) -> Self {
        Self { status_code: 405, message: message.into(), data: None }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self { status_code: 409, message: message.into(), data: None }
    }

    pub fn unprocessable_entity(message: impl Into<String>) -> Self {
        Self { status_code: 422, message: message.into(), data: None }
    }

    pub fn too_many_requests(message: impl Into<String>) -> Self {
        Self { status_code: 429, message: message.into(), data: None }
    }

    // ---- 5xx Server Errors (no data) ----
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self { status_code: 500, message: message.into(), data: None }
    }

    pub fn not_implemented(message: impl Into<String>) -> Self {
        Self { status_code: 501, message: message.into(), data: None }
    }

    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self { status_code: 503, message: message.into(), data: None }
    }

    // ---- Generic status with optional data ----
    pub fn with_status(status: StatusCode, message: impl Into<String>, data: Option<T>) -> Self {
        Self { status_code: status.as_u16(), message: message.into(), data }
    }

    pub fn err(status: StatusCode, message: impl Into<String>) -> Self {
        Self::with_status(status, message, None)
    }
}


impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let status = StatusCode::from_u16(self.status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        HttpResponse::build(status).json(self)
    }
}