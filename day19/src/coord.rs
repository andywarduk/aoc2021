use impl_ops::*;
use std::ops;

pub type CoordVal = i16;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Coord {
    pub a: CoordVal,
    pub b: CoordVal,
    pub c: CoordVal
}

impl Coord {

    pub fn new(a: CoordVal, b: CoordVal, c: CoordVal) -> Self {
        Self { a, b, c }
    }

    pub fn manhattan(&self) -> CoordVal {
        self.a.abs() + self.b.abs() + self.c.abs()
    }

}

impl std::fmt::Display for Coord {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[{},{},{}]", self.a, self.b, self.c)
    }

}

impl From<&Vec<CoordVal>> for Coord {

    fn from(cvec: &Vec<CoordVal>) -> Self {
        Self {
            a: cvec[0],
            b: cvec[1],
            c: cvec[2]
        }
    }

}

impl From<[CoordVal; 3]> for Coord {

    fn from(cvec: [CoordVal; 3]) -> Self {
        Self {
            a: cvec[0],
            b: cvec[1],
            c: cvec[2]
        }
    }

}

impl_op_ex!(+ |a: &Coord, b: &Coord| -> Coord {
    Coord {
        a: a.a + b.a,
        b: a.b + b.b,
        c: a.c + b.c
    }
});

impl_op_ex!(- |a: &Coord, b: &Coord| -> Coord {
    Coord {
        a: a.a - b.a,
        b: a.b - b.b,
        c: a.c - b.c
    }
});
