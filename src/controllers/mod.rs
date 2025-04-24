use actix_web::{get, HttpResponse, Responder};

mod user_controller;
pub use user_controller::*;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json("Welcome to Actix Web API")
}
