use std::f64::MIN;

use image::{GrayAlphaImage, GenericImageView};

pub type Coordinates = (u16, u16);

const MINIMUM_TRANSPARENCY: u8 = 0;

pub trait Transparent {
    fn is_transparent(&self, coords: Coordinates) -> bool;
}

impl Transparent for GrayAlphaImage {
    fn is_transparent(&self, coords: Coordinates) -> bool {
        self.get_pixel(coords.0 as u32, coords.1 as u32).0[1] > MINIMUM_TRANSPARENCY
    }
}

pub trait SameTone {
    fn same_tone(&self, coords: Coordinates, tone: u8) -> bool;
}

impl SameTone for GrayAlphaImage {
    fn same_tone(&self, coords: Coordinates, tone: u8) -> bool {
        self.get_pixel(coords.0 as u32, coords.1 as u32).0[0] == tone
    }
}