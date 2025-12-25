#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Vendor {
    pub id_vendor: i32,
    pub name_vendor: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Material {
    pub id_material: i32,
    pub name_material: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id_product: i32,
    pub id_vendor: i32,
    pub id_material: i32,
    pub name_product: String,
    pub diameter: f64,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Filament {
    pub id_filament: i32,
    pub id_product: i32,
    pub price: f64,
    pub color_name: String,
    pub color_hex: String,
    pub original_weight: i32,
    pub netto_weight: i32,
    pub spool_weight: i32,
    pub last_update: chrono::NaiveDateTime,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct ProductFull {
    pub id_product: i32,
    pub name_product: String,
    pub id_vendor: i32,
    pub name_vendor: String,
    pub id_material: i32,
    pub name_material: String,
    pub diameter: f64,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct FilamentFull {
    pub id_filament: i32,
    pub id_product: i32,
    pub name_product: String,
    pub id_vendor: i32,
    pub name_vendor: String,
    pub id_material: i32,
    pub name_material: String,
    pub diameter: f64,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
    pub price: f64,
    pub color_name: String,
    pub color_hex: String,
    pub original_weight: i32,
    pub netto_weight: i32,
    pub spool_weight: i32,
    pub last_update: chrono::NaiveDateTime,
}
