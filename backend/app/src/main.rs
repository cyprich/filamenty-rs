use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::NormalizePath, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod endpoints;
use crate::{api_docs::ApiDoc, endpoints::*};

mod api_docs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = lib::db::create_pool().await;

    // run migrations
    if let Err(e) = lib::db::run_migrations(&pool).await {
        panic!("{e}");
    }

    // prepare folders and filse
    lib::ensure_directory("images/uploads");
    lib::ensure_directory("images/qr");
    lib::qr::prepare_missing(&pool).await;

    //
    let port = env::var("BACKEND_PORT")
        .expect("'BACKEND_PORT' environment variable has to be set")
        .parse::<u16>()
        .expect("Couldn't convert 'BACKEND_PORT' to u16");

    let openapi = ApiDoc::openapi();

    // run the server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v2")
                    .wrap(NormalizePath::new(
                        actix_web::middleware::TrailingSlash::Trim,
                    ))
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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
