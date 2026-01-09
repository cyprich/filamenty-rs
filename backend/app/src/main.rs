use std::path::PathBuf;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::NormalizePath, web};

pub mod endpoints;
use crate::endpoints::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = lib::db::create_pool().await;

    lib::qr::prepare_missing(&pool).await;
    lib::ensure_directory(&PathBuf::from("images/uploads"));

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .wrap(Cors::permissive()) // TODO
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v2")
                    .service(hello)
                    .service(get_vendors)
                    .service(get_materials)
                    .service(get_products)
                    .service(get_products_full)
                    .service(get_filaments)
                    .service(get_filaments_full)
                    .service(get_filaments_by_id)
                    .service(get_last_update)
                    .service(post_vendors)
                    .service(post_materials)
                    .service(post_products)
                    .service(post_filaments)
                    .service(post_images)
                    .service(delete_vendors_by_id)
                    .service(delete_materials_by_id)
                    .service(delete_products_by_id)
                    .service(delete_filaments_by_id)
                    .service(patch_vendors_by_id)
                    .service(patch_materials_by_id)
                    .service(patch_products_by_id)
                    .service(patch_filaments_by_id)
                    .service(images),
            )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
