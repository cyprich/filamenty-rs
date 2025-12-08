use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::NormalizePath, web};

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/filaments")]
async fn get_filaments() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/filaments/{id}")]
async fn get_filaments_by_id(id: web::Path<usize>) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/info")]
async fn info() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .service(
                web::scope("/api/v2")
                    .service(hello)
                    .service(get_filaments)
                    .service(get_filaments_by_id)
                    .service(info),
            )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
