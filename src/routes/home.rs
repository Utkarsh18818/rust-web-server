#[warn(dead_code)]
use actix_web::web;
use crate::routes::logging;


pub fn home_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(home_handler));
}

async fn home_handler() -> &'static str {
    logging("GET:/home");
    "Welcome to the home page!"
}

