use actix_web::{App, HttpServer , web::Data};
mod routes;
use routes::{home_config, hello_user_config};

mod database;
use database::*;

#[warn(unused_imports)]
use crate::routes::create_user::create_user_config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database_pool = database_connection().await.expect("failed to connect to database");
    println!("Database connection established");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database_pool.clone())) // corrected variable name
            .configure(home_config)
            .configure(hello_user_config)
            .configure(create_user_config)
    })
    .bind(("127.0.0.1", 8000))?
    .run();

    println!("Server is running on 127.0.0.1:8000");
    server.await?;
    Ok(())
}
