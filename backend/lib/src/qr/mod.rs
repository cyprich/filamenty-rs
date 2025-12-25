use std::{
    env,
    path::{Path, PathBuf},
};

use dotenvy::dotenv;
use image::Luma;
use qrcode::QrCode;

use crate::{Filament, Pool};

pub async fn generate_manually(content: &str, filename: PathBuf) {
    let code = QrCode::new(content).expect("Failed to create QR code");

    // let qr = code
    //     .render::<char>()
    //     .dark_color('#')
    //     .light_color(' ')
    //     .module_dimensions(2, 1)
    //     .build();
    //
    // println!("{qr}");

    let image = code.render::<Luma<u8>>().quiet_zone(false).build();
    image.save(filename).expect("Failed to save image to file");
}

pub async fn generate_from_filament(filament: &Filament) {
    dotenv().ok();

    let dir = String::from("images");

    if !Path::new(&dir).exists() {
        std::fs::create_dir(&dir).expect("Failed to create directory `images`");
    };

    let filename = format!("qr{}.png", filament.id_filament);
    let filename = PathBuf::from(dir).join(filename);

    let hostname = env::var("HOSTNAME").expect("HOSTNAME environment variable has to be set");
    let content = format!("{hostname}/filament/{}", filament.id_filament);

    generate_manually(&content, filename).await;
}

pub async fn prepare_all(pool: &Pool) {
    let filaments = crate::db::get_filaments(pool)
        .await
        .expect("Failed to load filaments from database");

    for f in filaments {
        generate_from_filament(&f).await;
    }
}
