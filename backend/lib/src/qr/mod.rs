use std::{env, path::PathBuf};

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

    let filename = format!("{}.png", crate::uuid::get());
    let path = PathBuf::from(format!("images/qr/{filename}"));

    let hostname = env::var("HOSTNAME").expect("'HOSTNAME' environment variable has to be set");
    let content = format!("{hostname}/filament/{}", id);

    generate_manually(&content, path).await;

    format!("qr/{filename}")
}

pub async fn prepare_missing(pool: &Pool) {
    let missing_ids = crate::db::get_missing_qrcodes(pool)
        .await
        .expect("Failed to load IDs from database");

    for id in missing_ids {
        let qr_filename = generate_for_filament_id(id).await;
        crate::db::general_patch(
            pool,
            "filament",
            "qr_path",
            Some(&qr_filename),
            "id_filament",
            id,
            vec!["qr_path"],
        )
        .await
        .expect("Failed to update filaments in database");
    }
}
