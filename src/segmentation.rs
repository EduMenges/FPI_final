use std::{
    collections::{HashMap, LinkedList, BTreeMap},
    ops::RangeInclusive,
};

use image::{GenericImageView, GrayAlphaImage};

use crate::helpers::{Coordinates, Transparent, SameTone};

pub type ImageSegment = HashMap<u16, Vec<RangeInclusive<u16>>>;
pub type ImageSegments = Vec<ImageSegment>;

pub trait Connected {
    fn is_connected(&self, other: &Self) -> bool;
}

impl Connected for RangeInclusive<u16> {
    fn is_connected(&self, other: &Self) -> bool {
        if self.end() + 1 == *other.start() {
            true
        } else {
            other.end() + 1 == *self.start()
        }
    }
}

impl Connected for Vec<RangeInclusive<u16>> {
    fn is_connected(&self, other: &Self) -> bool {
        self.iter().any(|range| {
            other
                .iter()
                .any(|other_range| range.is_connected(other_range))
        })
    }
}

impl Connected for ImageSegment {
    fn is_connected(&self, other: &Self) -> bool {
        let common_ys = self.iter().filter(|(y, _)| other.contains_key(y));

        let mut res = common_ys.clone().any(|(y, ranges)| {
            let other_ranges = other.get(y).unwrap();
            ranges.is_connected(other_ranges)
        });

        if !res {
            if let Some((min_row, ranges)) = common_ys.clone().min_by_key(|(y, _)| **y) {
                // Necessário para não chegar diminuir a coordenada 0
                if let Some(upper_row) = min_row.checked_sub(1) {
                    if let Some(other_ranges) = other.get(&upper_row) {
                        res = ranges.is_connected(other_ranges);
                    }
                }
            }
        }

        if !res {
            if let Some((max_row, ranges)) = common_ys.max_by_key(|(y, _)| **y) {
                if let Some(other_ranges) = other.get(&(max_row + 1)) {
                    res = ranges.is_connected(other_ranges);
                }
            }
        }

        res
    }
}

pub trait EuclideanDistance {
    fn calc_euclidean_distance(&self, other: &Self) -> f64 {
        let centr_s = self.calc_centroid();
        let centr_o = other.calc_centroid();

        let x_dist = centr_s.0.abs_diff(centr_o.0) as f64;
        let y_dist = centr_s.1.abs_diff(centr_o.1) as f64;

        (x_dist.powi(2) + y_dist.powi(2)).sqrt()
    }

    fn calc_centroid(&self) -> Coordinates;
}

pub trait Crop {
    fn crop(&self, other: Self) -> Self;
}

impl Crop for ImageSegments {
    fn crop(&self, other: Self) -> Self {
        other
            .into_iter()
            .filter(|o_seg| self.iter().any(|s_seg| s_seg.is_connected(o_seg)))
            .collect()
    }
}

enum Direction {
    Left,
    Right,
}

pub struct ImgSegmentation<'a> {
    pub visited: VisitedPixels,
    pub segments: ImageSegments,
    img: &'a GrayAlphaImage,
}

impl<'a> ImgSegmentation<'a> {
    pub fn segment_img(img: &'a GrayAlphaImage) -> ImageSegments {
        let segments = ImageSegments::new();
        let visited = VisitedPixels::new((img.width() as u16, img.height() as u16));

        let mut this = Self {
            visited,
            segments,
            img,
        };

        for (x, y, _) in this.img.enumerate_pixels() {
            let coords = (x as u16, y as u16);

            if !this.visited.is_visited(coords) {
                this.visited.visit_tone(coords);

                let mut new_segment = ImageSegment::new();
                this.mount_segment(&mut new_segment, coords);

                if !this.img.is_transparent(coords) {
                    this.segments.push(new_segment);
                }
            }
        }

        this.segments
    }

    fn mount_segment(&mut self, new_segment: &mut ImageSegment, coords: Coordinates) {
        let tone_range = self.mount_line(coords);
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
        if !self.visited.is_visited(coords) && self.img.same_tone(coords, tone) && !self.img.is_transparent(coords) {
            self.visited.visit_tone(coords);
            self.mount_segment(new_segment, coords);
        }
    }

    fn mount_line(&mut self, coords: Coordinates) -> RangeInclusive<u16> {
        let lower = self.side_scan(coords, Direction::Left);
        let upper = self.side_scan(coords, Direction::Right);

        lower..=upper
    }

    fn side_scan(&mut self, coords: Coordinates, direction: Direction) -> u16 {
        let tone = self.img.get_pixel(coords.0 as u32, coords.1 as u32).0[0];

        let walk = match direction {
            Direction::Left => Box::from_iter((0..coords.0).rev()),
            Direction::Right => Box::from_iter((coords.0 + 1)..self.img.width() as u16),
        };

        let mut res = coords.0;

        for x in walk.iter() {
            let coords: Coordinates = (*x, coords.1);

            if self.img.same_tone(coords, tone) && !self.img.is_transparent(coords) {
                self.visited.visit_tone(coords);
                res = *x;
            } else {
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
