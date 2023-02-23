use std::{
    collections::{HashMap, LinkedList},
    ops::RangeInclusive,
};

use image::{GenericImageView, GrayAlphaImage};

pub type ImageSegment = HashMap<u16, Vec<RangeInclusive<u16>>>;
pub type ImageSegments = LinkedList<ImageSegment>;
pub type Coordinates = (u16, u16);

trait Connected {
    fn is_connected(&self, other: &Self) -> bool;
}

impl Connected for RangeInclusive<u16> {
    fn is_connected(&self, other: &Self) -> bool {
        if self.end() + 1 >= *other.start() {
            true
        } else if other.end() + 1 >= *self.start() {
            true
        } else {
            false
        }
    }
}

impl Connected for ImageSegment {
    fn is_connected(&self, other: &Self) -> bool {
        for (_, range) in other.iter().filter(|(k, _)| self.contains_key(*k)) {
            for r in range {
                if r.is_connected() {};
            }
        }

        false
    }
}

enum Direction {
    Left,
    Right,
}

pub struct ImgSegmentation<'a> {
    pub visited: VisitedPixels,
    pub segments: ImageSegments,
    pub pixel_stack: Vec<Coordinates>,
    img: &'a GrayAlphaImage,
}

impl<'a> ImgSegmentation<'a> {
    pub fn segment_img(img: &'a GrayAlphaImage) -> ImageSegments {
        let segments = ImageSegments::new();
        let visited = VisitedPixels::new((img.width() as u16, img.height() as u16));
        let pixel_stack: Vec<Coordinates> = Vec::new();
        let mut this = Self {
            visited,
            segments,
            pixel_stack,
            img,
        };

        for i in (0..img.height() as u16).rev() {
            this.pixel_stack.push((0, i));
        }

        while !this.pixel_stack.is_empty() {
            let coords = this.pixel_stack.pop().unwrap();

            if !this.visited.is_visited(coords) {
                this.visited.visit_tone(coords);
                let mut new_segment = ImageSegment::new();
                this.mount_segment(&mut new_segment, coords);
                this.segments.push_front(new_segment);
            }
        }

        this.segments
    }

    fn mount_segment(&mut self, new_segment: &mut ImageSegment, coords: Coordinates) {
        let tone_range = self.tone_to_range(coords);
        let tone = self.img.get_pixel(coords.0 as u32, coords.1 as u32).0[0];

        new_segment
            .entry(coords.1)
            .and_modify(|v| v.push(tone_range.clone()))
            .or_insert_with(|| vec![tone_range.clone()]);

        if coords.1 > 0 {
            for x in tone_range.clone() {
                self.mount_next_line((x, coords.1 - 1), tone, new_segment);
            }
        }
        if (coords.1 as u32) < self.img.height() - 1 {
            for x in tone_range {
                self.mount_next_line((x, coords.1 + 1), tone, new_segment);
            }
        }
    }

    fn mount_next_line(&mut self, coords: (u16, u16), tone: u8, new_segment: &mut ImageSegment) {
        if !self.visited.is_visited(coords) {
            let next_pixel = self.img.get_pixel(coords.0 as u32, coords.1 as u32).0;
            if next_pixel[0] == tone && next_pixel[1] > 0 {
                self.visited.visit_tone(coords);
                self.mount_segment(new_segment, coords);
            }
        }
    }

    fn tone_to_range(&mut self, coords: Coordinates) -> RangeInclusive<u16> {
        let lower = self.side_scan(coords, Direction::Left);
        let upper = self.side_scan(coords, Direction::Right);

        lower..=upper
    }

    fn side_scan(&mut self, coords: Coordinates, direction: Direction) -> u16 {
        let tone = self.img.get_pixel(coords.0 as u32, coords.1 as u32).0;

        let walk = match direction {
            Direction::Left => Box::from_iter((0..coords.0).rev()),
            Direction::Right => Box::from_iter((coords.0 + 1)..self.img.width() as u16),
        };

        let mut res = coords.0;

        for x in walk.into_iter() {
            let neighbour = self.img.get_pixel((*x).into(), coords.1 as u32);

            if neighbour.0[0] == tone[0] {
                self.visited.visit_tone((*x, coords.1));

                if neighbour.0[1] > 0 {
                    res = *x;
                } else {
                    break;
                }
            } else {
                self.pixel_stack.push((*x, coords.1));
                break;
            }
        }

        res
    }
}

pub struct VisitedPixels {
    visited: Vec<bool>,
    dimensions: Coordinates,
}

impl VisitedPixels {
    pub fn new(dimensions: Coordinates) -> Self {
        Self {
            visited: vec![false; dimensions.0 as usize * dimensions.1 as usize],
            dimensions,
        }
    }

    pub fn visit_tone(&mut self, coords: Coordinates) {
        self.visited[coords.0 as usize + (coords.1 * self.dimensions.0) as usize] = true;
    }

    pub fn is_visited(&self, coords: Coordinates) -> bool {
        self.visited[coords.0 as usize + (coords.1 * self.dimensions.0) as usize]
    }
}

#[cfg(test)]
mod tests {
    use image::io::Reader;

    use super::ImgSegmentation;

    #[test]
    fn test_segmentation() {
        let img = Reader::open(r"img_segments\segments.tif")
            .unwrap()
            .decode()
            .unwrap()
            .to_luma_alpha8();

        let segments = ImgSegmentation::segment_img(&img);

        assert_eq!(segments.len(), 8);
    }
}
