use actix_web::{Responder, patch, web};

use crate::endpoints::handle_general_result;

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct PatchRequestBody {
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
            "image_path",
        ],
    )
    .await;

    handle_general_result(result)
}
