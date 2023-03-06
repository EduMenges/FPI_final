use crate::helpers::{normalize_tone, Centroid, SmallCoord};

use super::Segment;

impl Centroid for Segment {
    fn calc_centroid(&self, img: &image::GrayAlphaImage) -> crate::helpers::CoordinatesF {
        let weighted_area = self.iter().fold(0.0, |area, (y, ranges)| {
            ranges.iter().fold(area, |acc, range| {
                range.clone().fold(acc, |acc, x| {
                    acc + normalize_tone(img.get_pixel_s((x, *y))[1])
                })
            })
        });

        let mut x_c = 0.0;
        let mut y_c = 0.0;

        for (y, ranges) in self.iter() {
            let (row_sum, row_w_avg) = ranges.iter().fold((0.0, 0.0), |(row_sum, row_w_avg), range| {
                range.clone().fold((row_sum, row_w_avg), |(row_sum, row_w_avg), x| {
                    let alpha = normalize_tone(img.get_pixel_s((x, *y))[1]);
                    (row_sum + alpha, row_w_avg + (x as f64) * alpha)
                })
            });

            x_c += row_w_avg / weighted_area;
            y_c += row_sum * ((*y as f64) / weighted_area);
        }

        (x_c, y_c)
    }
}
