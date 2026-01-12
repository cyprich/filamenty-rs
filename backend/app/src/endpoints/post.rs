use std::fs;

use actix_multipart::form::{MultipartForm, json::Json, tempfile::TempFile};
use actix_web::{HttpResponse, Responder, mime, post, web};

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

// //////////////////
// MULTIPART FORMS //
// //////////////////

// TODO dorobit do OpenAPI docs

// https://github.com/actix/actix-web/blob/5c6a29f4/actix-multipart/README.md
// https://docs.rs/utoipa/latest/utoipa/derive.ToSchema.html

fn verify_content_type(content_type: &str) -> Result<(), HttpResponse> {
    let allowed_types = ["image/png", "image/jpg", "image/jpeg", "image/webp"];

    match allowed_types.contains(&content_type) {
        true => Ok(()),
        false => {
            Err(HttpResponse::BadRequest().json(format!("Content type {content_type} not allowed")))
        }
    }
}

// returns Ok(new filename) or Err(error value)
fn save_tempfile_image(tempfile: &TempFile) -> Result<String, String> {
    let old_path = tempfile.file.path();
    let extension = tempfile.content_type.clone().unwrap().to_string();
    let extension = extension.split("/").last().unwrap();

    let new_filename = format!("{}.{}", lib::uuid::get(), extension);
    let new_path = format!("images/uploads/{new_filename}");

    match fs::copy(old_path, new_path) {
        Ok(_) => Ok(format!("uploads/{}", new_filename)),
        Err(val) => Err(val.to_string()),
    }
}

#[derive(Debug, MultipartForm, utoipa::ToSchema)]
pub struct ImageMultipartForm {
    #[schema(value_type = String, format = Binary, content_media_type = "image/png, image/jpg, image/webp")]
    image: TempFile,
}

#[post("/images")]
async fn post_images(MultipartForm(form): MultipartForm<ImageMultipartForm>) -> impl Responder {
    let content_type = form
        .image
        .content_type
        .clone()
        .unwrap_or(mime::TEXT_PLAIN)
        .to_string();

    if let Err(val) = verify_content_type(content_type.as_str()) {
        return val;
    }

    match save_tempfile_image(&form.image) {
        Ok(val) => HttpResponse::Created().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val),
    }
}

#[derive(Debug, MultipartForm, utoipa::ToSchema)]
pub struct FilamentMultipartForm {
    #[schema(value_type = String, format = Binary, content_media_type = "image/png, image/jpg, image/webp")]
    image: TempFile,
    #[schema(value_type = String, format = "Json", content_media_type = "application/json")]
    json: Json<NewFilament>,
}

#[post("/filaments-with-image")]
async fn post_filaments_with_image(
    pool: web::Data<lib::Pool>,
    MultipartForm(form): MultipartForm<FilamentMultipartForm>,
) -> impl Responder {
    let content_type = form
        .image
        .content_type
        .clone()
        .unwrap_or(mime::TEXT_PLAIN)
        .to_string();

    if let Err(val) = verify_content_type(content_type.as_str()) {
        return val;
    }

    let filename = match save_tempfile_image(&form.image) {
        Ok(val) => val,
        Err(val) => return HttpResponse::InternalServerError().json(val),
    };

    let mut filament = form.json.into_inner();
    filament.image_path = Some(filename);

    let result = lib::db::add_filaments(pool.get_ref(), filament).await;
    handle_type_result(result, crate::endpoints::TypeResultResponse::Created)
}
