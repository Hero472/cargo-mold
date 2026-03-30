use actix_web::{web, Responder};
use cargo_smith::common::response::ApiResponse;
use crate::features::{{name}}::service;
use crate::features::{{name}}::model::{{name_pascal_case}};

pub async fn get_all() -> impl Responder {
    let data = service::get_all().await;
    ApiResponse::ok("{{name}} retrieved successfully", data)
}

pub async fn get_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match service::get_by_id(&id).await {
        Some(item) => ApiResponse::ok(format!("{{name_pascal_case}} {} retrieved", id), item),
        None => ApiResponse::not_found(format!("{{name_pascal_case}} {} not found", id)),
    }
}

pub async fn create(body: web::Json<{{name_pascal_case}}>) -> impl Responder {
    let new_item = service::create(body.into_inner()).await;
    ApiResponse::created("{{name}} created", new_item)
}