use actix_web::web;
use crate::controllers::{
    index,
    list_jeux, create_jeu, update_jeu, toggle_active
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(list_jeux)
        .service(create_jeu)
        .service(update_jeu)
        .service(toggle_active);
}
