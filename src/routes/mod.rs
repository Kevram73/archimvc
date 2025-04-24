use actix_web::web;
use crate::controllers::{
    index,
    create_user, delete_user, get_user, get_users, update_user
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(get_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user);
}
