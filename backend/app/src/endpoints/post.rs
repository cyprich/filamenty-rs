use std::fs;

use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{HttpResponse, Responder, post, web};

use lib::db::models::{NewFilament, NewMaterial, NewProduct, NewVendor};

use crate::endpoints::handle_type_result;

#[post("/vendors")]
async fn post_vendors(pool: web::Data<lib::Pool>, vendor: web::Json<NewVendor>) -> impl Responder {
    let result = lib::db::add_vendors(pool.get_ref(), vendor.into_inner()).await;
    handle_type_result(result, crate::endpoints::TypeResultResponse::Created)
}

#[post("/materials")]
async fn post_materials(
    pool: web::Data<lib::Pool>,
    material: web::Json<NewMaterial>,
) -> impl Responder {
    let result = lib::db::add_materials(pool.get_ref(), material.into_inner()).await;
    handle_type_result(result, crate::endpoints::TypeResultResponse::Created)
}

#[post("/products")]
async fn post_products(
    pool: web::Data<lib::Pool>,
    product: web::Json<NewProduct>,
) -> impl Responder {
    let result = lib::db::add_products(pool.get_ref(), product.into_inner()).await;
    handle_type_result(result, crate::endpoints::TypeResultResponse::Created)
}

#[post("/filaments")]
async fn post_filaments(
    pool: web::Data<lib::Pool>,
    filament: web::Json<NewFilament>,
) -> impl Responder {
    let result = lib::db::add_filaments(pool.get_ref(), filament.into_inner()).await;
    handle_type_result(result, crate::endpoints::TypeResultResponse::Created)
}

// https://github.com/actix/actix-web/blob/5c6a29f4/actix-multipart/README.md
// https://docs.rs/utoipa/latest/utoipa/derive.ToSchema.html
#[derive(Debug, MultipartForm, utoipa::ToSchema)]
pub struct ImageMultipartForm {
    #[schema(value_type = String, format = Binary, content_media_type = "image/*")]
    file: TempFile,
}

#[post("/images")]
async fn post_images(MultipartForm(form): MultipartForm<ImageMultipartForm>) -> impl Responder {
    let allowed_content_types: Vec<String> = vec![
        String::from("image/png"),
        String::from("image/jpg"),
        String::from("image/jpeg"),
        String::from("image/webp"),
    ];

    let content_type = &form.file.content_type.clone().unwrap().to_string();

    if !allowed_content_types.contains(content_type) {
        return HttpResponse::BadRequest()
            .json(format!("Content type '{}' not allowed", content_type));
    }

    let old_path = &form.file.file.path();
    let extension = content_type.split('/').last().unwrap();
    let new_filename = format!("{}.{}", lib::uuid::get(), extension);
    let new_path = format!("images/uploads/{new_filename}");

    match fs::copy(old_path, new_path) {
        Ok(_) => HttpResponse::Created().json(format!("uploads/{}", new_filename)),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}
