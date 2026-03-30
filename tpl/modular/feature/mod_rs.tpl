pub mod controller;
pub mod service;
pub mod model;
pub mod routes;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    // This delegates route registration to the routes file
    routes::init(cfg);
}