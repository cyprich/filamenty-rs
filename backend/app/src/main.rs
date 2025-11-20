use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/api")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(hello))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
