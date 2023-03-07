use std::ops::RangeInclusive;

use crate::helpers::Overlaps;

use super::Segment;

impl Overlaps for RangeInclusive<u16> {
    #[inline]
    fn overlaps(&self, other: &Self) -> bool {
        self.end() >= other.start() && other.end() >= self.start()
    }
}

impl Overlaps for Vec<RangeInclusive<u16>> {
    #[inline]
    fn overlaps(&self, other: &Self) -> bool {
        self.iter()
            .any(|s_range| other.iter().any(|o_range| s_range.overlaps(o_range)))
    }
}

impl Overlaps for Segment {
    fn overlaps(&self, other: &Self) -> bool {
        self.iter().any(|(y, s_seg)| {
            other.iter().filter(|(y_o, _)| {
                *y == **y_o
            }).any(|(_, o_seg)| {
                s_seg.overlaps(o_seg)
            })
        })
    }
}