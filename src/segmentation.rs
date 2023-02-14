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

    let mut visited = vec![false; (img.width() * img.height()) as usize];

    let mut pixel_stack: Vec<Coordinates> = Vec::new();

    pixel_stack.push((0, 0));

    while !pixel_stack.is_empty() {
        let coords = pixel_stack.pop().unwrap();
        let tone = img.get_pixel(coords.0 as u32, coords.1 as u32).0;
    }

    segments
}

struct VisitedPixels {
    visited: Vec<bool>,
    dimensions: Coordinates,
}

impl VisitedPixels {
    pub fn new(dimensions: Coordinates) -> Self {
        Self { visited: Vec::with_capacity(dimensions.0 as usize * dimensions.1 as usize), dimensions }
    }

    pub fn visit_tone(&mut self, coords: Coordinates) {
        self.visited[coords.0 as usize + (coords.1 * self.dimensions.0) as usize] = true;
    }
}
