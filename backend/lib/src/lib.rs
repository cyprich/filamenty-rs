pub mod db;
pub mod qr;

pub use crate::db::Pool;

pub use crate::db::models::Filament;
pub use crate::db::models::FilamentFull;
pub use crate::db::models::Material;
pub use crate::db::models::Product;
pub use crate::db::models::ProductFull;
pub use crate::db::models::Vendor;

#[derive(Debug)]
pub enum Error {
    DatabaseError,
    DatabaseReferentialIntegrity,
    DatabaseDuplicate,
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
            Error::NotFound => {
                write!(f, "Not Found")
            }
        }
    }
}
