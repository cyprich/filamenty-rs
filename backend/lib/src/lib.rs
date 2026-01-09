pub mod db;
pub mod qr;
pub mod uuid;

use std::path::Path;
use std::path::PathBuf;

pub use crate::db::Pool;

pub use crate::db::models::Filament;
pub use crate::db::models::FilamentFull;
pub use crate::db::models::Material;
pub use crate::db::models::Product;
pub use crate::db::models::ProductFull;
pub use crate::db::models::Vendor;

#[derive(Debug, serde::Serialize)]
pub enum Error {
    DatabaseError,
    DatabaseReferentialIntegrity,
    DatabaseDuplicate,
    DatabaseRelationNotExist,
    DatabaseNullConstraint,
    DatabaseInvalidValue,
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::DatabaseError => {
                write!(f, "Error while communicating with database")
            }
            Error::DatabaseReferentialIntegrity => {
                write!(f, "Error with referential integrity in database")
            }
            Error::DatabaseDuplicate => {
                write!(f, "Given key already exists")
            }
            Error::DatabaseRelationNotExist => {
                write!(f, "Relation does not exist")
            }
            Error::DatabaseNullConstraint => {
                write!(f, "This value cannot be NULL")
            }
            Error::DatabaseInvalidValue => {
                write!(f, "Relation doesn't have given key")
            }
            Error::NotFound => {
                write!(f, "Not Found")
            }
        }
    }
}

pub fn ensure_directory(path: &PathBuf) {
    let dir = path.to_str().expect("Invalid directory path");
    if !Path::new(dir).exists() {
        std::fs::create_dir(dir).unwrap_or_else(|_| panic!("Failed to create directory {dir}"))
    };
}
