use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, get, web};

use crate::endpoints::handle_type_result;

#[get("")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/vendors")]
pub async fn get_vendors(pool: web::Data<lib::Pool>) -> impl Responder {
    let result = lib::db::get_vendors(pool.get_ref()).await;
    handle_type_result(result)
}

#[get("/materials")]
pub async fn get_materials(pool: web::Data<lib::Pool>) -> impl Responder {
    let result = lib::db::get_materials(pool.get_ref()).await;
    handle_type_result(result)
}

#[derive(serde::Deserialize)]
pub struct ProductQuery {
    id_vendor: Option<i32>,
}

#[get("/products")]
pub async fn get_products(
    pool: web::Data<lib::Pool>,
    query: web::Query<ProductQuery>,
) -> impl Responder {
    let result = lib::db::get_products(pool.get_ref(), query.id_vendor).await;
    handle_type_result(result)
}

#[get("/products_full")]
pub async fn get_products_full(
    pool: web::Data<lib::Pool>,
    query: web::Query<ProductQuery>,
) -> impl Responder {
    let result = lib::db::get_products_full(pool.get_ref(), query.id_vendor).await;
    handle_type_result(result)
}

#[get("/filaments")]
pub async fn get_filaments(pool: web::Data<lib::Pool>) -> impl Responder {
    let result = lib::db::get_filaments(pool.get_ref()).await;
    handle_type_result(result)
}

#[get("/filaments_full")]
pub async fn get_filaments_full(pool: web::Data<lib::Pool>) -> impl Responder {
    let result = lib::db::get_filaments_full(pool.get_ref()).await;
    handle_type_result(result)
}

#[get("/filaments/{id}")]
pub async fn get_filaments_by_id(pool: web::Data<lib::Pool>, id: web::Path<i32>) -> impl Responder {
    let result = lib::db::get_filaments_by_id(pool.get_ref(), id.into_inner()).await;
    handle_type_result(result)
}

#[get("/last_update")]
pub async fn get_last_update(pool: web::Data<lib::Pool>) -> impl Responder {
    let result = lib::db::get_last_update(pool.get_ref()).await;
    handle_type_result(result)
}

#[get("/images/{path:.*}")]
pub async fn images(path: web::Path<String>) -> actix_web::Result<NamedFile> {
    let path: PathBuf = PathBuf::from("images").join(path.into_inner());

    Ok(NamedFile::open(path)?)
}
