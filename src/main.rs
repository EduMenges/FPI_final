#![feature(iter_advance_by)]

mod camouflage;
mod graphs;
pub mod quantization;
pub mod segmentation;
pub mod helpers;
pub mod app;

use crate::app::CamouflageImages;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    
    eframe::run_native(
        "Camouflage Images",
        options,
        Box::new(|_cc| Box::<app::CamouflageImages>::default()),
    )
}