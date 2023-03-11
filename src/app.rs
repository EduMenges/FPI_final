use egui::{TopBottomPanel, menu, CentralPanel};
use image::{ImageBuffer, RgbaImage};

use self::{my_menu::open_image, image_wrapper::ImageWrapper};

pub mod image_wrapper;
pub mod my_menu;

#[derive(Default)]
pub struct CamouflageImages {
    background: Option<ImageWrapper>,
    foreground: Option<ImageWrapper>,
}

impl CamouflageImages {
    pub fn menu(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.menu_button("Open image", |ui| {
                        if ui.button("Background").clicked() {
                            let cu = open_image("Select an image for the background");
                            self.update_background(cu, ui.ctx());
                        };
                        if ui.button("Foreground").clicked() {

                        };
                    });
                });
            });
        });
    }

    pub fn central(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            if let Some(ref background) = self.background {
                ui.image(&background.texture, background.texture.size_vec2());
            }
        });
    }

    fn update_background(&mut self, new_img: Option<RgbaImage>, ctx: &egui::Context) {
        if let Some(img) = new_img {
            self.background = Some(ImageWrapper::new(img, String::from("background"), ctx));
        }
    }
}


impl eframe::App for CamouflageImages {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CamouflageImages::menu(self, ctx);
        CamouflageImages::central(self, ctx);
        // CentralPanel::default().show(ctx, |ui| {
        //     ui.ctx().load_texture("cu", egui::ColorImage::example(), Default::default());
        // });
    }
}
