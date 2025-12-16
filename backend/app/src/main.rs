use std::env;

use crate::endpoints::*;
use actix_web::{App, HttpServer, middleware::NormalizePath, web};

mod endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = lib::db::create_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v2")
                    .service(hello)
                    .service(get_filaments)
                    .service(get_filaments_by_id)
                    .service(get_materials)
                    .service(get_vendors)
                    .service(get_productlines)
                    .service(info),
            )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
