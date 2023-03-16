use std::path::Path;

use egui::Pos2;
use image::{GrayAlphaImage, io::Reader};

use crate::segmentation::{ImageSegments, ImgSegmentation};

pub type Coordinates = (u16, u16);
pub type CoordinatesF = (f64, f64);

const MINIMUM_TRANSPARENCY: u8 = 0;

pub trait Connected {
    fn is_connected(&self, other: &Self) -> bool;
}

pub trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}

pub trait Crop {
    fn crop(&self, other: Self) -> Self;
}

pub trait Centroid {
    fn calc_centroid(&self, img: &GrayAlphaImage) -> CoordinatesF;
}

pub trait Transparent {
    fn is_transparent(&self, coords: Coordinates) -> bool;
}

impl Transparent for GrayAlphaImage {
    #[inline]
    fn is_transparent(&self, coords: Coordinates) -> bool {
        self.get_pixel_s(coords)[1] == MINIMUM_TRANSPARENCY
    }
}

pub trait SameTone {
    fn same_tone(&self, coords: Coordinates, tone: u8) -> bool;
}

impl SameTone for GrayAlphaImage {
    #[inline]
    fn same_tone(&self, coords: Coordinates, tone: u8) -> bool {
        self.get_pixel_s(coords)[0] == tone
    }
}

#[inline]
pub fn normalize_tone(tone: u8) -> f64 {
    tone as f64 / u8::MAX as f64
}

pub trait SmallCoord {
    fn get_pixel_s(&self, coords: Coordinates) -> [u8; 2];
}

impl SmallCoord for GrayAlphaImage {
    #[inline]
    fn get_pixel_s(&self, coords: Coordinates) -> [u8; 2] {
        self.get_pixel(coords.0 as u32, coords.1 as u32).0
    }
}

pub fn img_to_segs<P> (path: P) -> ImageSegments
where P: AsRef<Path> {
    let img = Reader::open(path)
    .unwrap()
    .decode()
    .unwrap()
    .to_luma_alpha8();

    ImgSegmentation::segment_img(&img)
}

pub fn no_nonsense_sub(lhs: Pos2, rhs: Pos2) -> Pos2 {
    Pos2 { x: lhs.x - rhs.x, y: lhs.y - rhs.y }
}