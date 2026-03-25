use serde::Serialize;
use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::StatusCode};
use serde_json::json;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize = ()> {
    pub status_code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    /// 200 OK with Data
    pub fn ok(message: impl Into<String>, data: T) -> Self {
        Self {
            status_code: 200,
            message: message.into(),
            data: Some(data),
        }
    }

    /// 201 Created with Data
    pub fn created(message: impl Into<String>, data: T) -> Self {
        Self {
            status_code: 201,
            message: message.into(),
            data: Some(data),
        }
    }

    /// Generic message only (no data)
    pub fn message(status: StatusCode, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            status_code: status.as_u16(),
            message: message.into(),
            data: None,
        }
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

pub struct ApiMessage;

impl ApiMessage {
    pub fn ok(message: impl Into<String>) -> HttpResponse {
        HttpResponse::Ok().json(json!({
            "status_code": 200,
            "message": message.into(),
            "data": null
        }))
    }

    pub fn not_found(message: impl Into<String>) -> HttpResponse {
        HttpResponse::NotFound().json(json!({
            "status_code": 404,
            "message": message.into(),
            "data": null
        }))
    }
}