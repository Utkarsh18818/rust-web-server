#[warn(unused_imports)]
use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[get("/home")]
async fn home()-> impl Responder{
    let response: &str = "Welcome to actix web server";
    response
}
#[get("/hello/{firstname}/{lastname}")]
async fn hello_user(params: web::Path<(String, String)>) -> impl Responder {
    let response = User::new(params.0.clone(), params.1.clone());
    web::Json(response)
}

#[derive(Serialize)]
struct User{
    first_name:String,
    last_name:String
}
impl User {
    fn new(firstname:String , lastname:String)-> Self {
        Self{ first_name : firstname, last_name : lastname}
    }
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
