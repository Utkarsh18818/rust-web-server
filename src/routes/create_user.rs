
use actix_web::{http::StatusCode, post, web::{Json, ServiceConfig}, Responder, HttpResponse};
use serde::Serialize;
use crate::routes::logging;
use crate::routes::hello_user::User;

#[derive(Serialize)]
struct CreateUserResponse {
    id: i32,
    user: User,
}

#[post("/user/create")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    logging("POST: /user/create");

    let response = CreateUserResponse {
        id: 1,
        user: user.into_inner(),
    };

    HttpResponse::build(StatusCode::CREATED).json(response)
}

pub fn create_user_config(cfg: &mut ServiceConfig) {
    cfg.service(create_new_user);
}
