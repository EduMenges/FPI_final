use egui::Window;
use image::RgbaImage;

use super::image_wrapper::ImageWrapper;

pub struct Foreground {
    pub wrp: ImageWrapper,
    pub open: bool,
}

impl Foreground {
    pub fn new(img: RgbaImage, ctx: &egui::Context) -> Self {
        Self {
            wrp: ImageWrapper::new(img, String::from("foreground"), ctx),
            open: true,
        }
    }
}
