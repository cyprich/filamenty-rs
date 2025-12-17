use std::env;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::{Filament, Material, Product, Vendor};

pub mod models;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn create_pool() -> Pool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable has to be set");

    match PgPoolOptions::new().max_connections(5).connect(&url).await {
        Ok(val) => val,
        Err(_) => panic!("Could not create database pool"),
    }
}

pub async fn get_materials(pool: &Pool) -> Result<Vec<Material>, sqlx::Error> {
    sqlx::query_as::<_, Material>("select * from material")
        .fetch_all(pool)
        .await
}

pub async fn get_vendors(pool: &Pool) -> Result<Vec<Vendor>, sqlx::Error> {
    sqlx::query_as::<_, Vendor>("select * from vendor")
        .fetch_all(pool)
        .await
}

pub async fn get_products(pool: &Pool) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>("select * from product")
        .fetch_all(pool)
        .await
}

pub async fn get_filaments(pool: &Pool) -> Result<Vec<Filament>, sqlx::Error> {
    sqlx::query_as::<_, Filament>("select * from filament")
        .fetch_all(pool)
        .await
}
