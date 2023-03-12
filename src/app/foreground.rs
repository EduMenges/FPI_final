use egui::{Window, Vec2};
use image::RgbaImage;

use super::image_wrapper::ImageWrapper;

pub struct Foreground {
    pub wrp: ImageWrapper,
    pub open: bool,
    pub img: RgbaImage
}

impl Foreground {
    pub fn new(img: RgbaImage, ctx: &egui::Context, size: Vec2) -> Self {
        Self {
            wrp: ImageWrapper::new(img, String::from("foreground"), ctx),
            open: true,
            img: RgbaImage::new(size.x as _, size.y as _)
        }
    }
}
