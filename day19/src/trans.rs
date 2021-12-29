use itertools::Itertools;
use lazy_static::lazy_static;

use super::coord::{Coord, CoordVal};

#[derive(Debug)]
pub struct TransMatrix {
    name: String,
    matrix: Vec<Vec<CoordVal>>
}

impl TransMatrix {

    pub fn transform(&self, coord: &Coord) -> Coord {
        Coord {
            a: coord.a * self.matrix[0][0] + coord.b * self.matrix[0][1] + coord.c * self.matrix[0][2],
            b: coord.a * self.matrix[1][0] + coord.b * self.matrix[1][1] + coord.c * self.matrix[1][2],
            c: coord.a * self.matrix[2][0] + coord.b * self.matrix[2][1] + coord.c * self.matrix[2][2],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

}

lazy_static! {
    pub static ref TRANS_MATRICES: Vec<TransMatrix> = {
        let build_ment = |axis: &str, sign: &str| {
            let elem = "xyz".find(axis.chars().next().unwrap()).unwrap();
    
            let mut r = Vec::with_capacity(3);
            for _ in 0..elem { r.push(0) };
            r.push(if sign == "-" { -1 } else { 1 });
            for _ in elem..2 { r.push(0) };
            r
        };
    
        let sign_iter = vec![(0, "+++"), (1, "-++"), (1, "+-+"), (2, "--+"), (1, "++-"), (2, "-+-"), (2, "+--"), (3, "---")]
            .into_iter();
    
        let axis_iter = vec![(0, "xyz"), (1, "yxz"), (1, "zyx"), (1, "xzy"), (2, "yzx"), (2, "zxy")]
            .into_iter();

        axis_iter
            .cartesian_product(sign_iter)
            .filter(|((swaps, _), (negates, _))| (swaps + negates) % 2 == 0)
            .map(|((_, axes), (_, signs))| {
                let a = &axes[0..=0];
                let b = &axes[1..=1];
                let c = &axes[2..=2];
    
                let s1 = &signs[0..=0];
                let s2 = &signs[1..=1];
                let s3 = &signs[2..=2];

                let name = format!("{}{}{}{}{}{}", s1, a, s2, b, s3, c);
    
                let matrix = vec![
                    build_ment(a, &signs[0..=0]),
                    build_ment(b, &signs[1..=1]),
                    build_ment(c, &signs[2..=2])
                ];
    
                TransMatrix {
                    name,
                    matrix
                }
            })
            .collect()
    };
}
