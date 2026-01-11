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
            | lib::Error::DatabaseNullConstraint
            | lib::Error::MigrationError => HttpResponse::BadRequest().json(val.to_string()),
            lib::Error::NotFound
            | lib::Error::DatabaseRelationNotExist
            | lib::Error::DatabaseInvalidValue => HttpResponse::NotFound().json(val.to_string()),
            lib::Error::DatabaseDuplicate => HttpResponse::Conflict().json(val.to_string()),
            lib::Error::ZeroRowsAffected => HttpResponse::NotFound().json(val.to_string()),
        },
    }
}

pub enum TypeResultResponse {
    Ok,
    Created,
}

fn handle_type_result<T: serde::Serialize>(
    result: Result<T, lib::Error>,
    response_type: TypeResultResponse,
) -> HttpResponse {
    match result {
        Ok(val) => match response_type {
            TypeResultResponse::Ok => HttpResponse::Ok().json(val),
            TypeResultResponse::Created => HttpResponse::Created().json(val),
        },
        Err(val) => handle_general_result(Err(val)),
    }
}
