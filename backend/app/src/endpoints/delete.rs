use actix_web::{Responder, delete, web};

use crate::endpoints::handle_general_result;

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
