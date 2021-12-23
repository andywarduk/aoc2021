use std::ops::Range;

#[derive(Debug, Default, Clone)]
pub struct Cube {
    pub ranges: [Range<i32>; 3],
}

impl Cube {

    pub fn within_range(&self, xr: Range<i32>, yr: Range<i32>, zr: Range<i32>) -> bool {
        xr.start <= self.ranges[0].start &&
        xr.end >= self.ranges[0].end &&
        yr.start <= self.ranges[1].start &&
        yr.end >= self.ranges[1].end &&
        zr.start <= self.ranges[2].start &&
        zr.end >= self.ranges[2].end
    }

}