use serde::Serialize;
use actix_web::HttpResponse;
use serde_json::json;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub status_code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(message: impl Into<String>, data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            status_code: 200,
            message: message.into(),
            data: Some(data),
        })
    }

    pub fn created(message: impl Into<String>, data: T) -> HttpResponse {
        HttpResponse::Created().json(Self {
            status_code: 201,
            message: message.into(),
            data: Some(data),
        })
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