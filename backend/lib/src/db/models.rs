// //////////////////
// REGULAR STRUCTS //
// //////////////////

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Vendor {
    pub id_vendor: i32,
    pub name_vendor: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Material {
    pub id_material: i32,
    pub name_material: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    pub image_path: Option<String>,
    pub qr_path: Option<String>,
}

// ///////////////
// FULL STRUCTS //
// ///////////////

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    pub image_path: Option<String>,
    pub qr_path: Option<String>,
}

// //////////////
// NEW STRUCTS //
// //////////////

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct NewVendor {
    pub name_vendor: String,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct NewMaterial {
    pub name_material: String,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct NewProduct {
    pub id_vendor: i32,
    pub id_material: i32,
    pub name_product: String,
    pub diameter: Option<f64>,
    pub temp_min: i32,
    pub temp_max: Option<i32>,
    pub temp_bed_min: i32,
    pub temp_bed_max: Option<i32>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct NewFilament {
    pub id_product: i32,
    pub price: f64,
    pub color_name: String,
    pub color_hex: String,
    pub original_weight: i32,
    pub netto_weight: i32,
    pub spool_weight: i32,
    pub image_path: Option<String>,
}
