use std::ops::RangeInclusive;

use crate::helpers::Overlaps;

impl Overlaps for RangeInclusive<u16> {
    #[inline]
    fn overlaps(&self, other: &Self) -> bool {
        self.end() >= other.start() && other.end() >= self.start()
    }
}

impl Overlaps for Vec<RangeInclusive<u16>> {
    fn overlaps(&self, other: &Self) -> bool {
        self.iter()
            .any(|s_range| other.iter().any(|o_range| s_range.overlaps(o_range)))
    }
}
