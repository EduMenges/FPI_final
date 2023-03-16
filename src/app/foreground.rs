use egui::{Area, Context, Pos2, Vec2, Window};
use image::{imageops::overlay, RgbaImage};

use crate::helpers::{no_nonsense_sub, Coordinates};

use super::{image_wrapper::ImageWrapper, CamouflageImages};

pub struct Foreground {
    pub window: ImageWrapper, /// The interactable window that the user manipulates
    pub layer: ImageWrapper, /// The layer that covers the background
    pub open: bool,
    pos: Pos2,
}

impl Foreground {
    pub fn new(img: RgbaImage, ctx: &egui::Context, size: (u32, u32)) -> Self {
        Self {
            window: ImageWrapper::new(img, String::from("foreground_window"), ctx),
            layer: ImageWrapper::new(
                RgbaImage::new(size.0 as _, size.1 as _),
                String::from("foreground_wrp"),
                ctx,
            ),
            open: true,
            pos: CamouflageImages::DEFAULT_POS,
        }
    }

    #[inline]
    pub fn change_pos(&mut self, pos: Pos2) {
        self.pos = pos;
    }

    #[inline]
    fn correct_pos(&mut self) {
        self.pos = no_nonsense_sub(self.pos, CamouflageImages::DEFAULT_POS);
    }

    pub fn overlay(&mut self) {
        self.correct_pos();

        overlay(
            &mut self.layer.img,
            &self.window.scale_img(),
            self.pos.x as _,
            self.pos.y as _,
        );

        self.layer.reload_texture();
    }

    pub fn draw_foreground_layer(&mut self, ctx: &Context) {
        Area::new("foreground_layer")
            .fixed_pos(CamouflageImages::DEFAULT_POS)
            .movable(false)
            .show(ctx, |ui| {
                ui.image(&self.layer.texture, self.layer.size);
            });
    }

    pub fn reset_layer(&mut self, new_size: (u32, u32)) {
        let empty_image = RgbaImage::new(new_size.0, new_size.1);
        self.layer.update(empty_image);
    }

    #[inline]
    pub fn update(&mut self, img: RgbaImage) {
        self.window.update(img);
    }
}
