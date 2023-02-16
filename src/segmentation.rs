use std::{
    collections::{HashMap, LinkedList, VecDeque},
    ops::RangeInclusive,
};

use image::{GenericImageView, GrayAlphaImage};

pub type ImageSegment = HashMap<u16, Vec<RangeInclusive<u16>>>;
pub type ImageSegments = LinkedList<ImageSegment>;
pub type Coordinates = (u16, u16);

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

        this.pixel_stack.push((0, 0));

        while !this.pixel_stack.is_empty() {
            let coords = this.pixel_stack.pop().unwrap();

            if !this.visited.is_visited(coords) {
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

        if coords.1 != 0 && coords.1 as u32 != self.img.height() {
            for x in tone_range {
                self.mount_next_line((x, coords.1 - 1), tone, new_segment);
                self.mount_next_line((x, coords.1 + 1), tone, new_segment);
            }
        }
    }

    fn mount_next_line(&mut self, coords: (u16, u16), tone: u8, new_segment: &mut ImageSegment) {
        if !self.visited.is_visited(coords) {
            if self.img.get_pixel(coords.0 as u32, coords.1 as u32).0[0] == tone {
                self.mount_segment(new_segment, coords);
            } else {
                self.pixel_stack.push(coords);
            }
        }
    }

    fn tone_to_range(&mut self, coords: Coordinates) -> RangeInclusive<u16> {
        let lower = self.side_scan(coords, Direction::Left);
        let upper = self.side_scan(coords, Direction::Right);

        lower..=upper
    }

    fn side_scan(&mut self, coords: Coordinates, direction: Direction) -> u16 {
        let mut res = coords.0;
        let tone = self.img.get_pixel(coords.0 as u32, coords.1 as u32).0;
        let walk = match direction {
            Direction::Left => |coord| coord - 1,
            Direction::Right => |coord| coord + 1,
        };

        if tone[1] > 0 {
            loop {
                if res != 0 && res as u32 != self.img.width() {
                    res = walk(res);

                    let neighbour = self.img.get_pixel(res as u32, coords.1 as u32);

                    if neighbour.0[0] == tone[0] && neighbour.0[1] > 0 {
                        self.visited.visit_tone((res, coords.1));
                        res -= 1;
                    } else {
                        self.pixel_stack.push(coords);
                        break;
                    }
                } else {
                    break;
                }
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
            visited: vec![false; (dimensions.0 as usize * dimensions.1 as usize)],
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
