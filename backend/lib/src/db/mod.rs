use std::env;

use dotenvy::dotenv;
use sqlx::postgres::{PgPoolOptions, PgQueryResult};

use crate::{
    Filament, FilamentFull, Material, Product, ProductFull, Vendor,
    db::models::{NewFilament, NewMaterial, NewProduct, NewVendor},
};

pub mod models;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

// //////////
// GENERAL //
// //////////

pub async fn create_pool() -> Pool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable has to be set");

    match PgPoolOptions::new().max_connections(5).connect(&url).await {
        Ok(val) => val,
        Err(_) => panic!("Could not create database pool"),
    }
}

fn handle_general_result(result: Result<PgQueryResult, sqlx::Error>) -> Result<(), crate::Error> {
    match result {
        Ok(val) => {
            if val.rows_affected() != 0 {
                Ok(())
            } else {
                Err(crate::Error::ZeroRowsAffected)
            }
        }

        Err(val) => {
            if let sqlx::Error::Database(err) = val
                && err.code().is_some()
            {
                // error code 23502 = violates null constraint
                // error code 23503 = referential integrity
                // error code 23505 = duplicate key
                // error code 42P01 = relation does not exist

                match err.code().unwrap().trim() {
                    "23502" => Err(crate::Error::DatabaseNullConstraint),
                    "23503" => Err(crate::Error::DatabaseReferentialIntegrity),
                    "23505" => Err(crate::Error::DatabaseDuplicate),
                    "42P01" => Err(crate::Error::DatabaseRelationNotExist),
                    _ => Err(crate::Error::DatabaseError),
                }
            } else {
                Err(crate::Error::DatabaseError)
            }
        }
    }
}

pub async fn run_migrations(pool: &Pool) -> Result<(), crate::Error> {
    sqlx::migrate!("../migrations")
        .run(pool)
        .await
        .map_err(|_| crate::Error::MigrationError)
}

// ///////////////
// GET REQUESTS //
// ///////////////

pub async fn get_materials(pool: &Pool) -> Result<Vec<Material>, crate::Error> {
    sqlx::query_as::<_, Material>("select * from material order by id_material")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn get_vendors(pool: &Pool) -> Result<Vec<Vendor>, crate::Error> {
    sqlx::query_as::<_, Vendor>("select * from vendor order by id_vendor")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn get_products(pool: &Pool) -> Result<Vec<Product>, crate::Error> {
    sqlx::query_as::<_, Product>("select * from product order by id_product")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn get_filaments(pool: &Pool) -> Result<Vec<Filament>, crate::Error> {
    sqlx::query_as::<_, Filament>("select * from filament order by id_filament")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn get_products_full(pool: &Pool) -> Result<Vec<ProductFull>, crate::Error> {
    let sql = r#"
select * from product
join vendor using (id_vendor) 
join material using (id_material)
order by id_product
    "#;

    let sql = format!("{sql} where id_vendor = $1");

    sqlx::query_as::<_, ProductFull>(&sql)
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
order by id_filament
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
order by id_filament
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

pub async fn get_last_update(pool: &Pool) -> Result<chrono::NaiveDateTime, crate::Error> {
    let result = sqlx::query_scalar::<_, chrono::NaiveDateTime>(
        "select last_update from filament order by last_update desc limit 1",
    )
    .fetch_one(pool)
    .await
    .map_err(|_| crate::Error::DatabaseError)?;

    Ok(result)
}

pub async fn get_missing_qrcodes(pool: &Pool) -> Result<Vec<i32>, crate::Error> {
    sqlx::query_scalar::<_, i32>("select id_filament from filament where qr_path is null")
        .fetch_all(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

// ////////////////
// POST REQUESTS //
// ////////////////

pub async fn add_vendors(pool: &Pool, vendor: NewVendor) -> Result<Vendor, crate::Error> {
    let query = r#"
insert into vendor (name_vendor)
values ($1)
returning *
"#;

    sqlx::query_as::<_, Vendor>(query)
        .bind(vendor.name_vendor)
        .fetch_one(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn add_materials(pool: &Pool, material: NewMaterial) -> Result<Material, crate::Error> {
    let query = r#"
insert into material (name_material)
values ($1)
returning *
"#;

    sqlx::query_as::<_, Material>(query)
        .bind(material.name_material)
        .fetch_one(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn add_products(pool: &Pool, product: NewProduct) -> Result<Product, crate::Error> {
    let query = r#"
insert into product (
    id_vendor, id_material, 
    name_product, diameter,
    temp_min, temp_max, 
    temp_bed_min, temp_bed_max) 
values ($1, $2, $3, $4, $5, $6, $7, $8)
returning *
"#;

    sqlx::query_as::<_, Product>(query)
        .bind(product.id_vendor)
        .bind(product.id_material)
        .bind(product.name_product)
        .bind(product.diameter)
        .bind(product.temp_min)
        .bind(product.temp_max)
        .bind(product.temp_bed_min)
        .bind(product.temp_bed_max)
        .fetch_one(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

pub async fn add_filaments(pool: &Pool, filament: NewFilament) -> Result<Filament, crate::Error> {
    let query = r#"
insert into filament (
    id_product, price, 
    color_name, color_hex, 
    original_weight, netto_weight, spool_weight,
    image_path) 
values ($1, $2, $3, $4, $5, $6, $7, $8)
returning *
"#;

    let result = sqlx::query_as::<_, Filament>(query)
        .bind(filament.id_product)
        .bind(filament.price)
        .bind(filament.color_name)
        .bind(filament.color_hex)
        .bind(filament.original_weight)
        .bind(filament.netto_weight)
        .bind(filament.spool_weight)
        .bind(filament.image_path)
        .fetch_one(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)?;

    let qr_path = crate::qr::generate_for_filament_id(result.id_filament).await;

    let query = format!(
        "update filament set qr_path = '{}' where id_filament = {} returning *",
        qr_path, result.id_filament
    );

    sqlx::query_as(&query)
        .fetch_one(pool)
        .await
        .map_err(|_| crate::Error::DatabaseError)
}

// //////////////////
// DELETE REQUESTS //
// //////////////////

pub async fn general_delete_by_id(
    pool: &Pool,
    table: &str,
    id_name: &str,
    id_value: i32,
) -> Result<(), crate::Error> {
    let query = format!("delete from {table} where {id_name} = $1");

    let result = sqlx::query(&query).bind(id_value).execute(pool).await;

    handle_general_result(result)
}

// /////////////////
// PATCH REQUESTS //
// /////////////////

pub async fn general_patch(
    pool: &Pool,
    table: &str,
    variable_name: &str,
    variable_value: Option<&str>,
    id_name: &str,
    id_value: i32,
    valid_variable_names: Vec<&str>,
) -> Result<(), crate::Error> {
    if !valid_variable_names.contains(&variable_name) {
        return Err(crate::Error::DatabaseInvalidValue);
    }

    let variable_value = variable_value.unwrap_or("null");

    let variable_type = match variable_name {
        val if val.contains("name") || val.contains("path") => "text",
        "diameter" | "price" => "float",
        "color_hex" => "char(7)",
        "null" => "null",
        _ => "integer",
    };

    // https://neon.com/postgresql/postgresql-tutorial/postgresql-cast
    let query = format!(
        "update {table} set {variable_name} = $1::{variable_type}{} where {id_name} = $2",
        if table == "filament" {
            ", last_update = now()"
        } else {
            ""
        }
    );

    let result = sqlx::query(&query)
        .bind(variable_value)
        .bind(id_value)
        .execute(pool)
        .await;

    handle_general_result(result)
}
