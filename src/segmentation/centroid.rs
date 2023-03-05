// (somatório de x * valor de alpha) / (somatório de alpha)

use std::ops::RangeInclusive;

use image::GenericImageView;

use crate::helpers::Centroid;

use super::Segment;

impl Centroid for Segment {
    fn calc_centroid(&self, img: &image::GrayAlphaImage) -> crate::helpers::CoordinatesF {
        let weighted_area = self.iter().fold(0.0, |acc, (y, ranges)| {
            ranges
                .iter()
                .fold(acc, |acc, range| range.fold(acc, |acc, x| acc + normaliz))
        });

        let mut x_c = 0.0;
        let mut y_c = 0.0;

        for (y, ranges) in self.iter() {
            let (row_sum, next_x) = ranges.iter().fold((0.0, 0.0), |(row_sum, next_x), range| {
                range.fold((row_sum, next_x), |(row_sum, next_x), x| {
                    let alpha = (img.get_pixel(x as u32, *y as u32).0[0] as f64) / (u8::MAX as f64);
                    (row_sum + alpha, next_x + (x as f64) * alpha)
                })
            });

            x_c += next_x / weighted_area;
            y_c += row_sum * ((*y as f64) / weighted_area);
        }

        (x_c, y_c)
    }
}
