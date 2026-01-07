use std::{fs, path::PathBuf};

use actix_files::NamedFile;
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use lib::db::models::{NewFilament, NewMaterial, NewProduct, NewVendor};

// //////////
// GENERAL //
// //////////

fn handle_general_result(result: Result<(), lib::Error>) -> HttpResponse {
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(val) => match val {
            lib::Error::DatabaseError => HttpResponse::InternalServerError().json(val.to_string()),
            lib::Error::DatabaseReferentialIntegrity
            | lib::Error::DatabaseRelationNotExist
            | lib::Error::DatabaseInvalidValue
            | lib::Error::DatabaseNullConstraint => {
                HttpResponse::BadRequest().json(val.to_string())
            }
            lib::Error::DatabaseDuplicate => HttpResponse::Conflict().json(val.to_string()),
            lib::Error::NotFound => HttpResponse::NotFound().json(val.to_string()),
        },
    }
}

// ///////////////
// GET REQUESTS //
// ///////////////

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/vendors")]
async fn get_vendors(pool: web::Data<lib::Pool>) -> impl Responder {
    match lib::db::get_vendors(pool.get_ref()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/materials")]
async fn get_materials(pool: web::Data<lib::Pool>) -> impl Responder {
    match lib::db::get_materials(pool.get_ref()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[derive(serde::Deserialize)]
struct ProductQuery {
    id_vendor: Option<i32>,
}

#[get("/products")]
async fn get_products(
    pool: web::Data<lib::Pool>,
    query: web::Query<ProductQuery>,
) -> impl Responder {
    match lib::db::get_products(pool.get_ref(), query.id_vendor).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

#[get("/products_full")]
async fn get_products_full(
    pool: web::Data<lib::Pool>,
    query: web::Query<ProductQuery>,
) -> impl Responder {
    match lib::db::get_products_full(pool.get_ref(), query.id_vendor).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
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
            _ => HttpResponse::InternalServerError().json(val.to_string()),
        },
    }
}

#[get("/images/{name}")]
async fn images(name: web::Path<String>) -> actix_web::Result<NamedFile> {
    let path: PathBuf = PathBuf::from("images").join(name.into_inner());

    let named_file = NamedFile::open(path);

    if let Ok(file) = named_file {
        return Ok(file);
    }

    let fallback_path = PathBuf::from("images").join("missing_filament.png");
    let file = NamedFile::open(fallback_path);

    Ok(file?)
}

#[get("/last_update")]
async fn get_last_update(pool: web::Data<lib::Pool>) -> impl Responder {
    let result = lib::db::get_last_update(pool.get_ref()).await;
    match result {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().json(val),
    }
}

// ////////////////
// POST REQUESTS //
// ////////////////

#[post("/vendors")]
async fn post_vendors(pool: web::Data<lib::Pool>, vendor: web::Json<NewVendor>) -> impl Responder {
    let result = lib::db::add_vendors(pool.get_ref(), vendor.into_inner()).await;
    handle_general_result(result)
}

#[post("/materials")]
async fn post_materials(
    pool: web::Data<lib::Pool>,
    material: web::Json<NewMaterial>,
) -> impl Responder {
    let result = lib::db::add_materials(pool.get_ref(), material.into_inner()).await;
    handle_general_result(result)
}

#[post("/products")]
async fn post_products(
    pool: web::Data<lib::Pool>,
    product: web::Json<NewProduct>,
) -> impl Responder {
    let result = lib::db::add_products(pool.get_ref(), product.into_inner()).await;
    handle_general_result(result)
}

#[post("/filaments")]
async fn post_filaments(
    pool: web::Data<lib::Pool>,
    filament: web::Json<NewFilament>,
) -> impl Responder {
    let result = lib::db::add_filaments(pool.get_ref(), filament.into_inner()).await;
    handle_general_result(result)
}

// https://github.com/actix/actix-web/blob/5c6a29f4/actix-multipart/README.md
#[derive(Debug, MultipartForm)]
struct ImageMultipartForm {
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
    let new_path = format!("images/{new_filename}");

    match fs::copy(old_path, new_path) {
        Ok(_) => HttpResponse::Ok().json(new_filename),
        Err(val) => HttpResponse::InternalServerError().json(val.to_string()),
    }
}

// //////////////////
// DELETE REQUESTS //
// //////////////////

#[delete("/vendors/{id}")]
async fn delete_vendors_by_id(pool: web::Data<lib::Pool>, id: web::Path<i32>) -> impl Responder {
    let result =
        lib::db::general_delete_by_id(pool.get_ref(), "vendor", "id_vendor", id.into_inner()).await;
    handle_general_result(result)
}

#[delete("/materials/{id}")]
async fn delete_materials_by_id(pool: web::Data<lib::Pool>, id: web::Path<i32>) -> impl Responder {
    let result =
        lib::db::general_delete_by_id(pool.get_ref(), "material", "id_material", id.into_inner())
            .await;
    handle_general_result(result)
}

#[delete("/products/{id}")]
async fn delete_products_by_id(pool: web::Data<lib::Pool>, id: web::Path<i32>) -> impl Responder {
    let result =
        lib::db::general_delete_by_id(pool.get_ref(), "product", "id_product", id.into_inner())
            .await;
    handle_general_result(result)
}

#[delete("/filaments/{id}")]
async fn delete_filaments_by_id(pool: web::Data<lib::Pool>, id: web::Path<i32>) -> impl Responder {
    let result =
        lib::db::general_delete_by_id(pool.get_ref(), "filament", "id_filament", id.into_inner())
            .await;
    handle_general_result(result)
}

// /////////////////
// PATCH REQUESTS //
// /////////////////

#[derive(serde::Deserialize)]
struct PatchRequestBody {
    key: String,
    value: Option<String>,
}

#[patch("/vendors/{id}")]
async fn patch_vendors_by_id(
    pool: web::Data<lib::Pool>,
    id: web::Path<i32>,
    body: web::Json<PatchRequestBody>,
) -> impl Responder {
    let result = lib::db::general_patch(
        pool.get_ref(),
        "vendor",
        &body.key,
        body.value.as_deref(),
        "id_vendor",
        id.into_inner(),
        vec!["name_vendor"],
    )
    .await;

    handle_general_result(result)
}

#[patch("/materials/{id}")]
async fn patch_materials_by_id(
    pool: web::Data<lib::Pool>,
    id: web::Path<i32>,
    body: web::Json<PatchRequestBody>,
) -> impl Responder {
    let result = lib::db::general_patch(
        pool.get_ref(),
        "material",
        &body.key,
        body.value.as_deref(),
        "id_material",
        id.into_inner(),
        vec!["name_material"],
    )
    .await;

    handle_general_result(result)
}

#[patch("/products/{id}")]
async fn patch_products_by_id(
    pool: web::Data<lib::Pool>,
    id: web::Path<i32>,
    body: web::Json<PatchRequestBody>,
) -> impl Responder {
    let result = lib::db::general_patch(
        pool.get_ref(),
        "product",
        &body.key,
        body.value.as_deref(),
        "id_product",
        id.into_inner(),
        vec![
            "id_vendor",
            "id_material",
            "name_product",
            "diameter",
            "temp_min",
            "temp_max",
            "temp_bed_min",
            "temp_bed_max",
        ],
    )
    .await;

    handle_general_result(result)
}

#[patch("/filaments/{id}")]
async fn patch_filaments_by_id(
    pool: web::Data<lib::Pool>,
    id: web::Path<i32>,
    body: web::Json<PatchRequestBody>,
) -> impl Responder {
    let result = lib::db::general_patch(
        pool.get_ref(),
        "filament",
        &body.key,
        body.value.as_deref(),
        "id_filament",
        id.into_inner(),
        vec![
            "id_product",
            "price",
            "color_name",
            "color_hex",
            "original_weight",
            "netto_weight",
            "spool_weight",
        ],
    )
    .await;

    handle_general_result(result)
}
