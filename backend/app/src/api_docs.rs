use lib::db::models::*;
use utoipa::OpenApi;

use crate::endpoints::{ImageMultipartForm, PatchRequestBody};

#[derive(OpenApi)]
#[openapi(paths(
    hello,
    get_vendors,
    get_materials,
    get_products,
    get_products_full,
    get_filaments,
    get_filaments_by_id,
    get_filaments_full,
    get_last_update, 
    get_images,
    post_vendors,
    post_materials, 
    post_products,
    post_filaments,
    post_images,
    patch_vendors_by_id,
    patch_materials_by_id,
    patch_products_by_id,
    patch_filaments_by_id,
    delete_vendors_by_id,
    delete_materials_by_id,
    delete_products_by_id,
    delete_filaments_by_id
))]
pub struct ApiDoc;

// ///////////////
// GET REQUESTS //
// ///////////////

#[utoipa::path(get, path="/api/v2", responses(
    (status=200, description="Request successful")
))]
#[allow(dead_code)]
fn hello() {}

#[utoipa::path(get, path="/api/v2/vendors", responses(
    (status=200, description="Request successful", body = Vendor),
    (status=500, description="Error between backend and database"),
))]
#[allow(dead_code)]
fn get_vendors() {}

#[utoipa::path(get, path="/api/v2/materials", responses(
    (status=200, description="Request successful", body = Material),
    (status=500, description="Error between backend and database"),
))]
#[allow(dead_code)]
fn get_materials() {}

#[utoipa::path(get, path="/api/v2/products", responses(
    (status=200, description="Request successful", body = Product),
    (status=500, description="Error between backend and database"),
))]
#[allow(dead_code)]
fn get_products() {}

#[utoipa::path(get, path="/api/v2/products_full", responses(
    (status=200, description="Request successful", body = ProductFull),
    (status=500, description="Error between backend and database"),
))]
#[allow(dead_code)]
fn get_products_full() {}

#[utoipa::path(get, path="/api/v2/filaments", responses(
    (status=200, description="Request successful", body = Filament),
    (status=500, description="Error between backend and database"),
))]
#[allow(dead_code)]
fn get_filaments() {}

#[utoipa::path(
    get, 
    path="/api/v2/filaments/{id}", 
    params(("id" = i32, Path, description="ID of filament")),
    responses(
        (status=200, description="Request successful", body = FilamentFull),
        (status=404, description = "Filament not found"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn get_filaments_by_id() {}

#[utoipa::path(get, path="/api/v2/filaments_full", responses(
    (status=200, description="Request successful", body = FilamentFull),
    (status=500, description="Error between backend and database"),
))]
#[allow(dead_code)]
fn get_filaments_full() {}

#[utoipa::path(get, path="/api/v2/last_update", responses(
    (status=200, description="Request successful", body = NaiveDateTime),
    (status=500, description="Error between backend and database"),
))]
#[allow(dead_code)]
fn get_last_update() {}

#[utoipa::path(
    get, 
    path="/api/v2/images/{filename}", 
    params(("filename" = String, Path, description = "Filename of image to get"),),
    responses(
        (status=200, description="Request successful", body = String, content_type="image/*"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn get_images() {}

// ////////////////
// POST REQUESTS //
// ////////////////

#[utoipa::path(
    post, 
    path="/api/v2/vendors", 
    request_body = NewVendor,
    responses(
        (status=201, description="Successfully created", body = Vendor),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn post_vendors() {}

#[utoipa::path(
    post, 
    path="/api/v2/materials", 
    request_body = NewMaterial,
    responses(
        (status=201, description="Successfully created", body = Material),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn post_materials() {}

#[utoipa::path(
    post, 
    path="/api/v2/products", 
    request_body = NewProduct,
    responses(
        (status=201, description="Successfully created", body = Product),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn post_products() {}

#[utoipa::path(
    post, 
    path="/api/v2/filaments", 
    request_body = NewFilament,
    responses(
        (status=201, description="Successfully created", body = Filament),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn post_filaments() {}

#[utoipa::path(
    post, 
    path="/api/v2/images", 
    request_body = ImageMultipartForm,
    responses(
        (status=201, description="Successfully created", body = String),
        (status=400, description="Unallowed 'content-type'"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn post_images() {}

// /////////////////
// PATCH REQUESTS //
// /////////////////

#[utoipa::path(
    patch, 
    path="/api/v2/vendors/{id}", 
    params(("id" = String, Path),),
    request_body = PatchRequestBody,
    responses(
        (status=200, description="Request successful"),
        (status=400, description="Invalid database field name"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn patch_vendors_by_id() {}

#[utoipa::path(
    patch, 
    path="/api/v2/materials/{id}", 
    params(("id" = String, Path),),
    request_body = PatchRequestBody,
    responses(
        (status=200, description="Request successful"),
        (status=400, description="Invalid database field name"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn patch_materials_by_id() {}

#[utoipa::path(
    patch, 
    path="/api/v2/products/{id}", 
    params(("id" = String, Path),),
    request_body = PatchRequestBody,
    responses(
        (status=200, description="Request successful"),
        (status=400, description="Invalid database field name"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn patch_products_by_id() {}

#[utoipa::path(
    patch, 
    path="/api/v2/filaments/{id}", 
    params(("id" = String, Path),),
    request_body = PatchRequestBody,
    responses(
        (status=200, description="Request successful"),
        (status=400, description="Invalid database field name"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn patch_filaments_by_id() {}

// //////////////////
// DELETE REQUESTS //
// //////////////////

#[utoipa::path(
    delete, 
    path="/api/v2/vendors/{id}", 
    params(("id" = String, Path),),
    responses(
        (status=200, description="Request successful"),
        (status=404, description="Zero rows affected"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn delete_vendors_by_id() {}

#[utoipa::path(
    delete, 
    path="/api/v2/materials/{id}", 
    params(("id" = String, Path),),
    responses(
        (status=200, description="Request successful"),
        (status=404, description="Zero rows affected"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn delete_materials_by_id() {}

#[utoipa::path(
    delete, 
    path="/api/v2/products/{id}", 
    params(("id" = String, Path),),
    responses(
        (status=200, description="Request successful"),
        (status=404, description="Zero rows affected"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn delete_products_by_id() {}

#[utoipa::path(
    delete, 
    path="/api/v2/filaments/{id}", 
    params(("id" = String, Path),),
    responses(
        (status=200, description="Request successful"),
        (status=404, description="Zero rows affected"),
        (status=500, description="Error between backend and database"),
    )
)]
#[allow(dead_code)]
fn delete_filaments_by_id() {}
