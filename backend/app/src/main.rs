use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::NormalizePath, web};

mod endpoints;
use crate::endpoints::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = lib::db::create_pool().await;

    lib::qr::prepare_all(&pool).await;

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
                    .service(get_filaments)
                    .service(get_filaments_full)
                    .service(get_filaments_by_id)
                    .service(get_materials)
                    .service(delete_materials_by_id)
                    .service(get_vendors)
                    .service(get_products)
                    .service(get_products_full)
                    .service(delete_filaments_by_id)
                    .service(image)
                    .service(info),
            )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
