mod load;
mod coord;
mod trans;

use std::collections::BTreeMap;
use std::error::Error;
use std::collections::{HashMap, HashSet};

use coord::Coord;
use trans::TRANS_MATRICES;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let reports = load::load_input("input19.txt")?;

    let scanners = build_map(&reports);

    let points = build_points(&scanners);

    // Run parts
    part1(&points);
    part2(&scanners);

    Ok(())
}

fn part1(points: &[Coord]) {

    println!("Part 1: Number of beacons: {}", points.len());
}

fn part2(scanners: &[Scanner]) {
    let max_dist = scanners.iter().combinations(2).map(|scanner_vec| {
        (&scanner_vec[0].offset - &scanner_vec[1].offset).manhattan()
    }).max().unwrap();

    println!("Part 2: Maximum manhattan distance between scanners: {}", max_dist);
}

struct Scanner {
    offset: Coord,
    coords: Vec<Coord>,
}

fn build_map(reports: &[Vec<Coord>]) -> Vec<Scanner> {
    let mut solved: BTreeMap<usize, Scanner> = BTreeMap::new();

    solved.insert(0, Scanner {
        offset: Coord::new(0, 0, 0),
        coords: reports[0].clone(),
    });

    loop {
        let mut count = 0;

        for (e1, s1) in reports.iter().enumerate() {
            if solved.get(&e1).is_some() {
                continue
            }

            let mut new_coords: Option<(Vec<Coord>, Coord)> = None;

            for (e2, solved_ent) in solved.iter() {
                let s2 = &solved_ent.coords;

                for trans in &*TRANS_MATRICES {
                    let mut trans_map: HashMap<Coord, usize> = HashMap::with_capacity(reports.len());

                    for r1 in s1.iter().map(|r| trans.transform(r)) {
                        for r2 in s2 {
                            let diff = r2 - &r1;
                            *trans_map.entry(diff).or_insert(0) += 1;
                        }
                    }

                    if let Some(diff) = trans_map.iter().find(|(_, &cnt)| cnt >= 12).map(|(coord, _)| coord) {
                        println!("Solved {} -> {} ({})", e2, e1, diff);
                        new_coords = Some((
                            s1.iter().map(|r| diff + trans.transform(r)).collect(),
                            diff.clone()
                        ));
                        break
                    }
                }

                if new_coords.is_some() {
                    break
                }
            }

            if let Some((coords, offset)) = new_coords {
                solved.insert(e1, Scanner {
                    offset,
                    coords
                });

                count += 1;
            }
        }

        if solved.len() == reports.len() {
            break
        }

        if count == 0 {
            panic!("No solutions")
        }
    }

    solved.into_iter().map(|(_, solved)| solved).collect()
}

fn build_points(map: &[Scanner]) -> Vec<Coord> {
    let mut point_hash = HashSet::new();

    for s in map.iter() {
        for point in &s.coords {
            point_hash.insert(point);
        }
    }

    point_hash.into_iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let reports = load::load_buf(TEST_INPUT.as_bytes()).unwrap();

        let scanners = build_map(&reports);

        let mut points = build_points(&scanners);
        assert_eq!(points.len(), 79);
        points.sort();
        assert_eq!(points, TEST_COORDS);
    }

    const TEST_INPUT: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    const TEST_COORDS: [Coord; 79] = [
        Coord { a: -892, b: 524, c: 684 },
        Coord { a: -876, b: 649, c: 763 },
        Coord { a: -838, b: 591, c: 734 },
        Coord { a: -789, b: 900, c: -551 },
        Coord { a: -739, b: -1745, c: 668 },
        Coord { a: -706, b: -3180, c: -659 },
        Coord { a: -697, b: -3072, c: -689 },
        Coord { a: -689, b: 845, c: -530 },
        Coord { a: -687, b: -1600, c: 576 },
        Coord { a: -661, b: -816, c: -575 },
        Coord { a: -654, b: -3158, c: -753 },
        Coord { a: -635, b: -1737, c: 486 },
        Coord { a: -631, b: -672, c: 1502 },
        Coord { a: -624, b: -1620, c: 1868 },
        Coord { a: -620, b: -3212, c: 371 },
        Coord { a: -618, b: -824, c: -621 },
        Coord { a: -612, b: -1695, c: 1788 },
        Coord { a: -601, b: -1648, c: -643 },
        Coord { a: -584, b: 868, c: -557 },
        Coord { a: -537, b: -823, c: -458 },
        Coord { a: -532, b: -1715, c: 1894},
        Coord { a: -518, b: -1681, c: -600},
        Coord { a: -499, b: -1607, c: -770},
        Coord { a: -485, b: -357, c: 347},
        Coord { a: -470, b: -3283, c: 303},
        Coord { a: -456, b: -621, c: 1527},
        Coord { a: -447, b: -329, c: 318},
        Coord { a: -430, b: -3130, c: 366},
        Coord { a: -413, b: -627, c: 1469},
        Coord { a: -345, b: -311, c: 381},
        Coord { a: -36, b: -1284, c: 1171},
        Coord { a: -27, b: -1108, c: -65},
        Coord { a: 7, b: -33, c: -71},
        Coord { a: 12, b: -2351, c: -103},
        Coord { a: 26, b: -1119, c: 1091},
        Coord { a: 346, b: -2985, c: 342},
        Coord { a: 366, b: -3059, c: 397},
        Coord { a: 377, b: -2827, c: 367},
        Coord { a: 390, b: -675, c: -793},
        Coord { a: 396, b: -1931, c: -563},
        Coord { a: 404, b: -588, c: -901},
        Coord { a: 408, b: -1815, c: 803},
        Coord { a: 423, b: -701, c: 434},
        Coord { a: 432, b: -2009, c: 850},
        Coord { a: 443, b: 580, c: 662},
        Coord { a: 455, b: 729, c: 728},
        Coord { a: 456, b: -540, c: 1869},
        Coord { a: 459, b: -707, c: 401},
        Coord { a: 465, b: -695, c: 1988},
        Coord { a: 474, b: 580, c: 667},
        Coord { a: 496, b: -1584, c: 1900},
        Coord { a: 497, b: -1838, c: -617},
        Coord { a: 527, b: -524, c: 1933},
        Coord { a: 528, b: -643, c: 409},
        Coord { a: 534, b: -1912, c: 768},
        Coord { a: 544, b: -627, c: -890},
        Coord { a: 553, b: 345, c: -567},
        Coord { a: 564, b: 392, c: -477},
        Coord { a: 568, b: -2007, c: -577},
        Coord { a: 605, b: -1665, c: 1952},
        Coord { a: 612, b: -1593, c: 1893},
        Coord { a: 630, b: 319, c: -379},
        Coord { a: 686, b: -3108, c: -505},
        Coord { a: 776, b: -3184, c: -501},
        Coord { a: 846, b: -3110, c: -434},
        Coord { a: 1135, b: -1161, c: 1235},
        Coord { a: 1243, b: -1093, c: 1063},
        Coord { a: 1660, b: -552, c: 429},
        Coord { a: 1693, b: -557, c: 386},
        Coord { a: 1735, b: -437, c: 1738},
        Coord { a: 1749, b: -1800, c: 1813},
        Coord { a: 1772, b: -405, c: 1572},
        Coord { a: 1776, b: -675, c: 371},
        Coord { a: 1779, b: -442, c: 1789},
        Coord { a: 1780, b: -1548, c: 337},
        Coord { a: 1786, b: -1538, c: 337},
        Coord { a: 1847, b: -1591, c: 415},
        Coord { a: 1889, b: -1729, c: 1762},
        Coord { a: 1994, b: -1805, c: 1792},
    ];

}