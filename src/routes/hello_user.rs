use actix_web::{get, web, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    first_name: String,
    last_name: String,
}

impl User {
    fn new(firstname: String, lastname: String) -> Self {
        Self { first_name: firstname, last_name: lastname }
    }
}

#[get("/hello/{firstname}/{lastname}")]
pub async fn hello_user(params: web::Path<(String, String)>) -> impl Responder {
    let response: User = User::new(params.0.clone(), params.1.clone());
    web::Json(response)
}

pub fn hello_user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_user);
}

