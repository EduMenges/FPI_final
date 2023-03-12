#![feature(iter_advance_by)]
#![feature(duration_constants)]
pub mod app;
mod camouflage;
mod graphs;
pub mod helpers;
pub mod quantization;
pub mod segmentation;

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
