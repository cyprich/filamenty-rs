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
