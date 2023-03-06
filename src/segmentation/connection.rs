use crate::helpers::{Connected, Overlaps};
use crate::segmentation::Segment;
use std::ops::RangeInclusive;

impl Connected for RangeInclusive<u16> {
    #[inline]
    fn is_connected(&self, other: &Self) -> bool {
        if self.end() + 1 == *other.start() {
            true
        } else {
            other.end() + 1 == *self.start()
        }
    }
}

impl Connected for Vec<RangeInclusive<u16>> {
    #[inline]
    fn is_connected(&self, other: &Self) -> bool {
        self.iter().any(|range| {
            other
                .iter()
                .any(|other_range| range.is_connected(other_range))
        })
    }
}

impl Connected for Segment {
    fn is_connected(&self, other: &Self) -> bool {
        let common_ys = self.iter().filter(|(y, _)| other.contains_key(y));

        let mut res = common_ys.clone().any(|(y, ranges)| {
            let other_ranges = other.get(y).unwrap();
            ranges.is_connected(other_ranges)
        });

        if !res {
            let (min_row, ranges) = self.iter().min_by_key(|(y, _)| **y).unwrap();

            // Necessário para não diminuir a coordenada 0
            if let Some(upper_row) = min_row.checked_sub(1) {
                if let Some(other_ranges) = other.get(&upper_row) {
                    res = ranges.overlaps(other_ranges);
                }
            }
        }

        if !res {
            let (max_row, ranges) = self.iter().max_by_key(|(y, _)| **y).unwrap();

            if let Some(other_ranges) = other.get(&(max_row + 1)) {
                res = ranges.overlaps(other_ranges);
            }
        }

        res
    }
}
