use std::env;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::{Filament, FilamentFull, Material, Product, ProductFull, Vendor};

pub mod models;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

// GENERAL //

pub async fn create_pool() -> Pool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable has to be set");

    match PgPoolOptions::new().max_connections(5).connect(&url).await {
        Ok(val) => val,
        Err(_) => panic!("Could not create database pool"),
    }
}

async fn general_delete_by_id(pool: &Pool, id: i32, what: &str) -> Result<bool, crate::Error> {
    let query = format!("delete from {what} where id_{what} = $1");
    let result = sqlx::query(&query).bind(id).execute(pool).await;

    // error code 23503 = referential integrity

    match result {
        Ok(val) => Ok(val.rows_affected() != 0),
        Err(val) => {
            if let sqlx::Error::Database(err) = val
                && err.code().is_some()
                && err.code().unwrap() == "23503"
            {
                Err(crate::Error::DatabaseReferentialIntegrity)
            } else {
                Err(crate::Error::DatabaseError)
            }
        }
    }
}

// MATERIALS //

pub async fn get_materials(pool: &Pool) -> Result<Vec<Material>, crate::Error> {
    sqlx::query_as::<_, Material>("select * from material")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn delete_material_by_id(pool: &Pool, id: i32) -> Result<bool, crate::Error> {
    general_delete_by_id(pool, id, "material").await
}

// VENDORS //

pub async fn get_vendors(pool: &Pool) -> Result<Vec<Vendor>, crate::Error> {
    sqlx::query_as::<_, Vendor>("select * from vendor")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn delete_vendor_by_id(pool: &Pool, id: i32) -> Result<bool, crate::Error> {
    general_delete_by_id(pool, id, "vendor").await
}

// PRODUCTS //

pub async fn get_products(pool: &Pool) -> Result<Vec<Product>, crate::Error> {
    sqlx::query_as::<_, Product>("select * from product")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn get_products_full(pool: &Pool) -> Result<Vec<ProductFull>, crate::Error> {
    sqlx::query_as::<_, ProductFull>(
        r#"
    select * from product
    join vendor using (id_vendor) 
    join material using (id_material)
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|_| crate::Error::DatabaseError)
}

pub async fn delete_product_by_id(pool: &Pool, id: i32) -> Result<bool, crate::Error> {
    general_delete_by_id(pool, id, "vendor").await
}

// FILAMENTS //

pub async fn get_filaments(pool: &Pool) -> Result<Vec<Filament>, crate::Error> {
    sqlx::query_as::<_, Filament>("select * from filament")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn get_filaments_full(pool: &Pool) -> Result<Vec<FilamentFull>, crate::Error> {
    sqlx::query_as::<_, FilamentFull>(
        r#"
    select * from filament
    join product using (id_product)
    join vendor using (id_vendor) 
    join material using (id_material)
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|_| crate::Error::DatabaseError)
}

pub async fn get_filaments_by_id(pool: &Pool, id: i32) -> Result<FilamentFull, crate::Error> {
    sqlx::query_as::<_, FilamentFull>(
        r#"
    select * from filament
    join product using (id_product)
    join vendor using (id_vendor) 
    join material using (id_material)
    where id_filament = $1
    "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => crate::Error::NotFound,
        _ => crate::Error::DatabaseError,
    })
}

pub async fn delete_filament_by_id(pool: &Pool, id: i32) -> Result<bool, crate::Error> {
    general_delete_by_id(pool, id, "vendor").await
}
