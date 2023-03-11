use std::path::PathBuf;

use eframe::glow::LINEAR;
use egui::{TextureHandle, Context, ColorImage, TextureOptions};
use image::RgbaImage;

pub struct ImageWrapper {
    pub img: RgbaImage,
    pub texture: TextureHandle,
}

impl ImageWrapper {
    pub fn new(img: RgbaImage, name: String, ctx: &Context) -> Self {
        let samples = img.as_flat_samples();
        let rgba = samples.as_slice();
        let size = [img.width() as _, img.height() as _];
        let egui_image = ColorImage::from_rgba_unmultiplied(size, rgba);

        let texture = ctx.load_texture(name, egui_image, TextureOptions::LINEAR);

        Self { img, texture }
    }
}