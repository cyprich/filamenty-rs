use actix_web::{HttpResponse, Responder, get, web};

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/filaments")]
async fn get_filaments(pool: web::Data<lib::Pool>) -> impl Responder {
    let filaments = lib::db::get_filaments(pool.get_ref()).await;

    match filaments {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/filaments/{id}")]
async fn get_filaments_by_id(id: web::Path<usize>) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/materials")]
async fn get_materials(pool: web::Data<lib::Pool>) -> impl Responder {
    let materials = lib::db::get_materials(pool.get_ref()).await;

    match materials {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/vendors")]
async fn get_vendors(pool: web::Data<lib::Pool>) -> impl Responder {
    let vendors = lib::db::get_vendors(pool.get_ref()).await;

    match vendors {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/products")]
async fn get_productlines(pool: web::Data<lib::Pool>) -> impl Responder {
    let products = lib::db::get_products(pool.get_ref()).await;

    match products {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/info")]
async fn info() -> impl Responder {
    HttpResponse::NotImplemented()
}
