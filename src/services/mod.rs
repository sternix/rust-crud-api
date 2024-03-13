use actix_web::web;

mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    user::init(cfg);
}
