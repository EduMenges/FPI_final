use egui::{Pos2, Vec2, Window};
use image::{imageops::overlay, RgbaImage};

use super::{image_wrapper::ImageWrapper, CamouflageImages};

pub struct Foreground {
    pub window: ImageWrapper,
    pub wrp: ImageWrapper,
    pub open: bool,
    pos: Pos2,
}

impl Foreground {
    pub fn new(img: RgbaImage, ctx: &egui::Context, size: Vec2) -> Self {
        Self {
            window: ImageWrapper::new(img, String::from("foreground_window"), ctx),
            wrp: ImageWrapper::new(
                RgbaImage::new(size.x as _, size.y as _),
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

    pub fn overlay(&mut self) {
        self.window.scale_img();
        overlay(
            &mut self.wrp.img,
            &self.window.img,
            self.pos.x as _,
            self.pos.y as _,
        );
    }
}
