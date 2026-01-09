use actix_web::HttpResponse;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;

pub use delete::*;
pub use get::*;
pub use patch::*;
pub use post::*;

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

fn handle_type_result<T: serde::Serialize>(result: Result<T, lib::Error>) -> HttpResponse {
    match result {
        Ok(val) => HttpResponse::Created().json(val),
        Err(val) => handle_general_result(Err(val)),
    }
}
