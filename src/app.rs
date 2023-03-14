use std::{path::PathBuf, time::Duration};

use eframe::{epaint::Shadow, Frame};
use egui::{
    menu, CentralPanel, Color32, Margin, Pos2, Rounding, SidePanel, Stroke, TopBottomPanel, Window,
};
use image::{ImageBuffer, RgbaImage};

use self::{
    foreground::Foreground,
    image_wrapper::ImageWrapper,
    my_menu::{load_image, open_image},
};

pub mod foreground;
pub mod image_wrapper;
pub mod my_menu;

#[derive(Default)]
pub struct CamouflageImages {
    background: Option<ImageWrapper>,
    foreground: Option<Foreground>,
}

impl CamouflageImages {
    pub const DEFAULT_POS: Pos2 = Pos2 { x: 8.0, y: 32.0 };

    const FOREGROUND_MARGIN: Margin = Margin {
        left: 8.0,
        right: 8.0,
        top: 8.0,
        bottom: 8.0,
    };

    const NO_MARGIN: Margin = Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    };

    const FOREGROUND_FRAME: egui::containers::Frame = egui::containers::Frame {
        inner_margin: CamouflageImages::NO_MARGIN,
        outer_margin: CamouflageImages::FOREGROUND_MARGIN,
        rounding: Rounding {
            nw: 0.0,
            ne: 0.0,
            sw: 0.0,
            se: 0.0,
        },
        shadow: Shadow::NONE,
        fill: Color32::TRANSPARENT,
        stroke: Stroke {
            width: 0.5,
            color: Color32::GREEN,
        },
    };

    pub fn menu(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.menu_button("Open image", |ui| {
                        if ui.button("Background").clicked() {
                            let background = open_image("Select an image for the background");
                            self.update_background(background, ctx);
                            ui.close_menu();
                        };

                        if ui.button("Foreground").clicked() {
                            let foreground = load_image(PathBuf::from(r"img_segments\graph_1.png"));
                            self.update_foreground(foreground, ctx);
                            ui.close_menu();
                        };
                    });
                });
            });
        });
    }

    pub fn central(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([true, true])
                .auto_shrink([true, true])
                .min_scrolled_width(400.0)
                .show(ui, |ui| {
                    if let Some(ref background) = self.background {
                        ui.image(&background.texture, background.texture.size_vec2());
                    }
                });

            if let Some(ref mut foreground) = self.foreground {
                Window::new("Foreground")
                    .open(&mut foreground.open)
                    .resizable(true)
                    .title_bar(false)
                    .constrain(true)
                    .frame(CamouflageImages::FOREGROUND_FRAME)
                    .default_size(foreground.window.texture.size_vec2())
                    .show(ctx, |ui| {
                        foreground.window.scale_size(ui.available_width());
                        ui.image(&foreground.window.texture, foreground.window.size);
                    });
            }
        });
    }

    fn update_background(&mut self, new_img: Option<RgbaImage>, ctx: &egui::Context) {
            if let Some(img) = new_img {
                self.background = Some(ImageWrapper::new(img, String::from("background"), ctx));
            }
    }

    fn update_foreground(&mut self, new_img: Option<RgbaImage>, ctx: &egui::Context) {
        if let Some(ref background) = self.background {
            if let Some(img) = new_img {
                self.foreground = Some(Foreground::new(img, ctx, background.texture.size_vec2()));
            }
        }
    }

    pub fn side(&mut self, ctx: &egui::Context) {
        SidePanel::right("apply_menu").show(ctx, |ui| {
            if let Some(ref mut foreground) = self.foreground {
                if ui.button("Apply").clicked() {}
            }
        });
    }
}

impl eframe::App for CamouflageImages {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CamouflageImages::menu(self, ctx);
        CamouflageImages::side(self, ctx);
        CamouflageImages::central(self, ctx);
        
        ctx.request_repaint_after(Duration::SECOND);
        // CentralPanel::default().show(ctx, |ui| {
        //     ui.ctx().load_texture("cu", egui::ColorImage::example(), Default::default());
        // });
    }
}
