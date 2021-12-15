use std::collections::{HashMap};
use std::cmp::Reverse;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use memmap2::Mmap;
use priority_queue::PriorityQueue;

type Risk = u8;
type PathRisk = u32;
type Coord = u16;
type Coords = (Coord, Coord);

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let risks = load_input("input15.txt")?;

    // Run parts
    part1(&risks);

    let risks2 = expand_map(&risks);
    part2(&risks2);

    Ok(())
}

fn part1(risks: &[Vec<Risk>]) {
    let lowest_risk = find_lowest_risk(risks);

    println!("Part 1: Lowest risk: {:?}", lowest_risk);
}

fn part2(risks: &[Vec<Risk>]) {
    let lowest_risk = find_lowest_risk(risks);

    println!("Part 2: Lowest risk: {:?}", lowest_risk);
}

fn find_lowest_risk(risks: &[Vec<Risk>]) -> PathRisk {
    let width = risks[0].len();
    let height = risks.len();

    let mut prev: HashMap<Coords, Coords> = HashMap::new();
    let mut queue: PriorityQueue<Coords, Reverse<PathRisk>> = PriorityQueue::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let dist: PathRisk = if x == 0 && y == 0 { 0 } else { PathRisk::MAX };

            queue.push((x as Coord, y as Coord), Reverse(dist));
        }
    }

    let neighbours = |x, y| {
        let mut neigh = Vec::with_capacity(4);

        neigh.push((x + 1, y));
        neigh.push((x, y + 1));

        if x > 0 {
            neigh.push((x - 1, y));
        }

        if y > 0 {
            neigh.push((x, y - 1));
        }

        neigh
    };

    while let Some((item1 @ (x1, y1), Reverse(dist))) = queue.pop() {
        for item2 @ (x2, y2) in neighbours(x1, y1) {
            if let Some((_, &Reverse(cur_dist))) = queue.get(&item2) {
                let calc_dist = dist + risks[y2 as usize][x2 as usize] as PathRisk;
                if calc_dist < cur_dist {
                    queue.change_priority(&item2, Reverse(calc_dist));
                    *prev.entry(item2).or_default() = item1;
                }
            }
        }
    }

    // Walk backwards
    let mut cur_pos = ((width - 1) as Coord, (height - 1) as Coord);
    let mut risk: PathRisk = 0;

    while cur_pos != (0, 0) {
        risk += risks[cur_pos.1 as usize][cur_pos.0 as usize] as PathRisk;
        cur_pos = *prev.get(&cur_pos).unwrap();
    }

    risk
}

fn expand_map(risks: &[Vec<Risk>]) -> Vec<Vec<Risk>> {
    let mut new_map = Vec::with_capacity(risks.len() * 5);

    for ychunk in 0..5 {
        for srcline in risks {
            let mut new_line = Vec::with_capacity(srcline.len() * 5);
            for xchunk in 0..5 {
                for val in srcline {
                    new_line.push((((val + xchunk + ychunk) - 1) % 9) + 1);
                }
            }
            new_map.push(new_line);
        }
    }

    new_map
}

type ParseResult = Vec<Vec<Risk>>;

fn load_input(file: &str) -> Result<ParseResult, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

fn load_buf(buf: &[u8]) -> Result<ParseResult, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create returns
    let mut risks = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.is_empty() {
            continue;
        }

        risks.push(line.chars().map(|c| c as u8 - b'0').collect());
    }
   
    Ok(risks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    const EXPANDED_INPUT: &str = "\
11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";

    #[test]
    fn test_lowest_risk() {
        let risks1 = load_buf(TEST_INPUT.as_bytes()).unwrap();
        let risks2 = load_buf(EXPANDED_INPUT.as_bytes()).unwrap();

        let lowest_risk = find_lowest_risk(&risks1);

        assert_eq!(lowest_risk, 40);

        let test_risks2 = expand_map(&risks1);

        assert_eq!(risks2, test_risks2);

        let lowest_risk = find_lowest_risk(&risks2);

        assert_eq!(lowest_risk, 315);
    }

}
