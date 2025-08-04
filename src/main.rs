#[warn(unused_imports)]
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/home")]
async fn home()-> impl Responder{
    let response: &str = "Welcome to actix web server";
    response
}
#[get("/hello/{firstname}/{lastname}")]
async fn hello_user(params: crate::web::Path<(String,String)>)-> impl Responder{
    let response: String = format!("Hello {} {}",params.0 , params.1);
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| { App::new()
        .service(home)
        .service(hello_user)

    }).bind(("127.0.0.1" , 8000))?
    .run();
    println!("Server is running 127.0.0.1:7000");
    server.await.unwrap() ;
    Ok(())
}
