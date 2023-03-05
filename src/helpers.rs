use std::f64::MIN;

use image::{GenericImageView, GrayAlphaImage};

pub type Coordinates = (u16, u16);
pub type CoordinatesF = (f64, f64);

const MINIMUM_TRANSPARENCY: u8 = 0;

pub trait Connected {
    fn is_connected(&self, other: &Self) -> bool;
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
        self.get_pixel(coords.0 as u32, coords.1 as u32).0[1] > MINIMUM_TRANSPARENCY
    }
}

pub trait SameTone {
    fn same_tone(&self, coords: Coordinates, tone: u8) -> bool;
}

impl SameTone for GrayAlphaImage {
    #[inline]
    fn same_tone(&self, coords: Coordinates, tone: u8) -> bool {
        self.get_pixel(coords.0 as u32, coords.1 as u32).0[0] == tone
    }
}

pub fn normalize_tone(tone: u8) -> f64 {
    tone as f64 / u8::MAX as f64
}
