use actix_web::{HttpResponse, Responder, get, web};

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/filaments")]
async fn get_filaments() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/filaments/{id}")]
async fn get_filaments_by_id(id: web::Path<usize>) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/materials")]
async fn get_materials() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/vendors")]
async fn get_vendors() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/productlines")]
async fn get_productlines() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/info")]
async fn info() -> impl Responder {
    HttpResponse::NotImplemented()
}
