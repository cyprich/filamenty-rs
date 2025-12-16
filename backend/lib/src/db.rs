use std::env;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool() -> sqlx::Pool<sqlx::Postgres> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable has to be set");

    match PgPoolOptions::new().max_connections(5).connect(&url).await {
        Ok(val) => val,
        Err(_) => panic!("Could not create database pool"),
    }
}
