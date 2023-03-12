use std::path::PathBuf;

use eframe::glow::LINEAR;
use egui::{TextureHandle, Context, ColorImage, TextureOptions, Vec2};
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

    pub fn maintain_ratio(&self, available_width: f32) -> Vec2 {
        let size = self.texture.size();
        let increase_ratio = available_width / size[0] as f32;
        let new_height = increase_ratio * size[1] as f32;
        Vec2{ x: available_width, y: new_height }
    }
}