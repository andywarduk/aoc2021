use std::collections::HashSet;
use std::cmp;
use std::fmt;

type Coord = i16;
type Coords = (Coord, Coord);

pub struct Image {
    default: bool,
    pixels: HashSet<Coords>
}

impl Image {

    pub fn new(image: &[Vec<bool>]) -> Self {
        let mut pixels = HashSet::new();

        for (r, row) in image.iter().enumerate() {
            for (c, set) in row.iter().enumerate() {
                if *set {
                    pixels.insert((c as Coord, r as Coord));
                }
            }
        }

        Image {
            default: false,
            pixels
        }
    }

    fn size(&self) -> (Coords, Coords) {
        self.pixels
            .iter()
            .fold(((Coord::MAX, Coord::MAX), (Coord::MIN, Coord::MIN)), |((minx, miny), (maxx, maxy)), (px, py)| {
            ((cmp::min(minx, *px), cmp::min(miny, *py)), (cmp::max(maxx, *px), cmp::max(maxy, *py)))
        })
    }

    pub fn enhance(&self, algo: &[bool]) -> Self {
        let default = if self.default {
            algo[511]
        } else {
            algo[0]
        };

        let ((minx, miny), (maxx, maxy)) = self.size();

        let mut pixels = HashSet::new();

        let build_lookup = |px, py| -> usize {
            let mut index = 0;

            for y in (py - 1)..=(py + 1) {
                for x in (px - 1)..=(px + 1) {
                    index <<= 1;

                    if self.pixels.contains(&(x, y)) != self.default {
                        index |= 1;
                    }
                }
            }

            index
        };

        for x in (minx - 1)..=(maxx + 1) {
            for y in (miny - 1)..=(maxy + 1) {
                let lookup = build_lookup(x, y);
                
                if algo[lookup] != default {
                    pixels.insert((x, y));
                }
            }
        }

        Self {
            default,
            pixels
        }
    }

    pub fn count(&self) -> usize {
        self.pixels.len()
    }

}

impl fmt::Display for Image {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let ((minx, miny), (maxx, maxy)) = self.size();

        let mut output = String::new();

        for y in miny..=maxy {
            for x in minx..=maxx {
                let set = if self.pixels.contains(&(x, y)) {
                    !self.default
                } else {
                    self.default
                };

                if set {
                    output += "#"
                } else {
                    output += "."
                }
            }
            output += "\n";
        }

        write!(f, "{}", output)
    }

}
