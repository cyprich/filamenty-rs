use std::{
    env,
    path::{Path, PathBuf},
};

use dotenvy::dotenv;
use image::Luma;
use qrcode::QrCode;

use crate::Pool;

pub async fn generate_manually(content: &str, path: PathBuf) {
    let code = QrCode::new(content).expect("Failed to create QR code");

    let image = code.render::<Luma<u8>>().quiet_zone(false).build();
    image.save(path).expect("Failed to save image to file");
}

pub async fn generate_for_filament_id(id: i32) -> String {
    dotenv().ok();

    let dir = String::from("images");

    if !Path::new(&dir).exists() {
        std::fs::create_dir(&dir).expect("Failed to create directory `images`")
    };

    let filename = format!("{}.png", crate::uuid::get());
    let path = PathBuf::from(dir).join(&filename);

    let hostname = env::var("HOSTNAME").expect("'HOSTNAME' environment variable has to be set");
    let content = format!("{hostname}/filament/{}", id);

    generate_manually(&content, path).await;

    filename
}

pub async fn prepare_all(pool: &Pool) {
    let filaments = crate::db::get_filaments(pool)
        .await
        .expect("Failed to load filaments from database");

    for f in filaments {
        generate_for_filament_id(f.id_filament).await;
    }
}
