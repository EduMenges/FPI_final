use std::{
    collections::{HashMap, LinkedList, VecDeque},
    ops::Range,
};

use image::GrayAlphaImage;

pub type ImageSegment = HashMap<u16, Vec<Range<u16>>>;
pub type ImageSegments = LinkedList<ImageSegment>;
pub type Coordinates = (u16, u16);

pub fn segment_img(img: &GrayAlphaImage) -> ImageSegments {
    let mut segments = ImageSegments::new();

    let mut visited = VisitedPixels::new((img.width() as u16, img.height() as u16));

    let mut pixel_stack: Vec<Coordinates> = Vec::new();

    pixel_stack.push((0, 0));

    while !pixel_stack.is_empty() {
        let coords = pixel_stack.pop().unwrap();
    }

    segments
}

fn tone_to_range(img: &GrayAlphaImage, coords: Coordinates) -> Range<u16> {
    let mut lower = side_scan(img, coords, -1);
    let mut upper = side_scan(img, coords, 1);

    lower..=upper
}

fn side_scan(img: &GrayAlphaImage, coords: Coordinates, increment: i8) -> u16 {
    let mut res = coords.0;
    let tone = img.get_pixel(coords.0 as u32, coords.1 as u32).0;

    if tone[1] > 0 {
        loop {
            let left_neighbour =
                img.get_pixel_checked(coords.0 + increment as u32, coords.1 as u32);

            match left_neighbour {
                Some(pixel) => {
                    if pixel.0 == tone && pixel.1 > 0 {
                        res -= 1;
                    }
                }
                None => break,
            }
        }
    }

    res
}

struct VisitedPixels {
    visited: Vec<bool>,
    dimensions: Coordinates,
}

impl VisitedPixels {
    pub fn new(dimensions: Coordinates) -> Self {
        Self {
            visited: Vec::with_capacity(dimensions.0 as usize * dimensions.1 as usize),
            dimensions,
        }
    }

    pub fn visit_tone(&mut self, coords: Coordinates) {
        self.visited[coords.0 as usize + (coords.1 * self.dimensions.0) as usize] = true;
    }
}
