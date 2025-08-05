use actix_web::{App, HttpServer};
mod routes;
use routes::{home_config, hello_user_config};

#[warn(unused_imports)]
use crate::routes::{create_user::create_new_user, create_user_config};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
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
