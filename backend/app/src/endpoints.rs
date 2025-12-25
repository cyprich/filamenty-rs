use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, get, web};

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/filaments")]
async fn get_filaments(pool: web::Data<lib::Pool>) -> impl Responder {
    match lib::db::get_filaments(pool.get_ref()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/filaments_full")]
async fn get_filaments_full(pool: web::Data<lib::Pool>) -> impl Responder {
    match lib::db::get_filaments_full(pool.get_ref()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/filaments/{id}")]
async fn get_filaments_by_id(pool: web::Data<lib::Pool>, id: web::Path<i32>) -> impl Responder {
    match lib::db::get_filaments_by_id(pool.get_ref(), id.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => match val {
            lib::Error::NotFound => HttpResponse::NotFound().json(val.to_string()),
            lib::Error::DatabaseError => HttpResponse::InternalServerError().json(val.to_string()),
        },
    }
}

#[get("/materials")]
async fn get_materials(pool: web::Data<lib::Pool>) -> impl Responder {
    match lib::db::get_materials(pool.get_ref()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/vendors")]
async fn get_vendors(pool: web::Data<lib::Pool>) -> impl Responder {
    match lib::db::get_vendors(pool.get_ref()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/products")]
async fn get_products(pool: web::Data<lib::Pool>) -> impl Responder {
    match lib::db::get_products(pool.get_ref()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/products_full")]
async fn get_products_full(pool: web::Data<lib::Pool>) -> impl Responder {
    match lib::db::get_products_full(pool.get_ref()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/image/{name}")]
async fn image(name: web::Path<String>) -> actix_web::Result<NamedFile> {
    let path: PathBuf = PathBuf::from("images").join(name.into_inner());

    Ok(NamedFile::open(path)?)
}

#[get("/info")]
async fn info() -> impl Responder {
    HttpResponse::NotImplemented()
}
