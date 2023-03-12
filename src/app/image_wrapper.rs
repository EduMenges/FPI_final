use std::path::PathBuf;

use eframe::glow::LINEAR;
use egui::{ColorImage, Context, Pos2, TextureHandle, TextureOptions, Vec2};
use image::{imageops::resize, RgbaImage};

pub struct ImageWrapper {
    pub img: RgbaImage,
    pub texture: TextureHandle,
    pub size: Vec2,
}

impl ImageWrapper {
    pub fn new(img: RgbaImage, name: String, ctx: &Context) -> Self {
        let samples = img.as_flat_samples();
        let rgba = samples.as_slice();
        let size = [img.width() as _, img.height() as _];

        let egui_image = ColorImage::from_rgba_unmultiplied(size, rgba);

        let texture = ctx.load_texture(name, egui_image, TextureOptions::LINEAR);
        let size = Vec2 {
            x: size[0] as _,
            y: size[1] as _,
        };

        Self { img, texture, size }
    }

    pub fn scale_size(&mut self, new_width: f32) {
        let size = self.texture.size();
        let increase_ratio = new_width / size[0] as f32;
        let new_height = increase_ratio * size[1] as f32;

        self.size = Vec2 {
            x: new_width,
            y: new_height,
        };
    }

    pub fn scale_img(&mut self) {
        self.img = resize(
            &self.img,
            self.size.x as _,
            self.size.y as _,
            image::imageops::FilterType::CatmullRom,
        );
    }
}
