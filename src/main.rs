use actix_web::{App, HttpServer, Responder, get};

#[get("/hello")]
async fn hello_world() -> impl Responder {
    "Hello world"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello_world))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}