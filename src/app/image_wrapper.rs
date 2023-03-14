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
    const SCALLING_OPTIONS: TextureOptions = TextureOptions::LINEAR;

    pub fn new(img: RgbaImage, name: String, ctx: &Context) -> Self {
        let egui_image = Self::img_to_egui(&img);

        let texture = ctx.load_texture(name, egui_image, Self::SCALLING_OPTIONS);

        let size = Vec2 {
            x: size[0] as _,
            y: size[1] as _,
        };

        Self { img, texture, size }
    }

    fn img_to_egui(img: &RgbaImage) -> ColorImage {
        let samples = img.as_flat_samples();
        let rgba = samples.as_slice();
        let size = [img.width() as _, img.height() as _];

        let egui_image = ColorImage::from_rgba_unmultiplied(size, rgba);
    }

    pub fn scale_size(&mut self, new_width: f32) {
        let size = self.texture.size();
        let increase_ratio = new_width / (size[0] as f32);
        let new_height = increase_ratio * (size[1] as f32);

        self.size = Vec2 {
            x: new_width,
            y: new_height,
        };
    }

    pub fn scale_img(&mut self) -> RgbaImage {
        resize(
            &self.img,
            self.size.x as _,
            self.size.y as _,
            image::imageops::FilterType::CatmullRom,
        )
    }

    pub fn update(&mut self, img: RgbaImage) {
        self.img = img;
        self.texture.set(Self::img_to_egui(&img), Self::SCALLING_OPTIONS);
    }
}
