use actix_web::{get, HttpResponse, Responder};

mod jeu_controller;
pub use jeu_controller::*;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json("Welcome to Actix Web API")
}
