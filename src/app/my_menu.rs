use std::path::PathBuf;

use image::RgbaImage;
use rfd::FileDialog;

fn load_dialog(title: &str) -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Image", &["png", "jpg", "jpeg", "gif", "webp"])
        .add_filter("Other files", &["*"])
        .set_title(title)
        .pick_file()
}

pub fn load_image(path: PathBuf) -> Option<RgbaImage> {
    let img = image::io::Reader::open(path).unwrap().decode();
    match img {
        Ok(img) => Some(img.into_rgba8()),
        Err(_) => None,
    }
}

pub fn open_image(title: &str) -> Option<RgbaImage> {
    let path = load_dialog(title);
    match path {
        Some(path) => load_image(path),
        None => None,
    }
}
