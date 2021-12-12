use std::collections::VecDeque;
use std::rc::Rc;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let conns = load_input("input12.txt")?;

    // Build tree
    let tree = build_tree(&conns);

    // Run parts
    part1(&tree);
    part2(&tree);

    Ok(())
}

fn part1(tree: &HashMap<String, Vec<String>>) {
    let paths = count_paths1(tree);

    println!("Part 1: Number of paths: {}", paths);
}

fn part2(tree: &HashMap<String, Vec<String>>) {
    let paths = count_paths2(tree);

    println!("Part 2: Number of paths: {}", paths);
}

#[derive(Clone)]
struct Path1<'a> {
    visited: Rc<HashSet<&'a str>>,
    pos: &'a str
}

fn count_paths1(tree: &HashMap<String, Vec<String>>) -> usize {
    let mut paths: usize = 0;

    let mut work_paths: VecDeque<Path1> = VecDeque::new();
    
    work_paths.push_back(Path1 {
        visited: Rc::new(HashSet::new()),
        pos: "start"
    });

    while let Some(work_path) = work_paths.pop_front() {
        let choices = tree.get(work_path.pos).unwrap();

        for choice in choices {
            if choice == "end" {
                // Reached the end
                paths += 1;
                continue
            }
            
            let visited;

            if lowercase_string(choice) {
                if work_path.visited.get(&choice[..]).is_some() {
                    // Already visited this cave
                    continue
                }

                // Mark this cave as visited
                let mut new_visited = HashSet::with_capacity(work_path.visited.len() + 1);
                new_visited.clone_from(&*work_path.visited);
                new_visited.insert(choice);
                visited = Rc::new(new_visited);
            } else {
                visited = work_path.visited.clone();
            }

            work_paths.push_back(Path1 {
                visited,
                pos: &choice[..]
            });
        }
    }

    paths
}

#[derive(Debug, Clone, PartialEq)]
enum SmallVisit<'a> {
    None,
    VisitedOnce(&'a String),
    VisitedTwice
}

#[derive(Debug, Clone)]
struct Path2<'a> {
    small_visit: SmallVisit<'a>,
    dont_visit: Rc<HashSet<&'a str>>,
    pos: &'a str
}

fn count_paths2(tree: &HashMap<String, Vec<String>>) -> usize {
    let mut paths: usize = 0;

    let mut work_paths: VecDeque<Path2> = VecDeque::new();
    
    work_paths.push_back(Path2 {
        small_visit: SmallVisit::None,
        dont_visit: Rc::new(HashSet::new()),
        pos: "start"
    });

    while let Some(work_path) = work_paths.pop_front() {
        let choices = tree.get(work_path.pos).unwrap();

        for choice in choices {
            if choice == "end" {
                // Reached the end
                match work_path.small_visit {
                    SmallVisit::VisitedOnce(_) => {
                        // Ignore this solution - small cave only visited once
                    }
                    _ => {
                        paths += 1;
                    }
                }

                continue
            }
            
            if lowercase_string(choice) {
                if work_path.dont_visit.get(&choice[..]).is_some() {
                    continue
                }

                // Emit two paths -
                // one where this small cave is visited twice and
                // one where the small cave is visited once
                if work_path.small_visit == SmallVisit::None || work_path.small_visit == SmallVisit::VisitedOnce(choice) {
                    let dont_visit;
    
                    let small_visit = if work_path.small_visit == SmallVisit::None {
                        // Visit for the first time
                        dont_visit = work_path.dont_visit.clone();
                        
                        SmallVisit::VisitedOnce(choice)
                    } else {
                        // Second visit
                        let mut new_dont_visit = HashSet::with_capacity(work_path.dont_visit.len() + 1);
                        new_dont_visit.clone_from(&*work_path.dont_visit);
                        new_dont_visit.insert(choice);
                        dont_visit = Rc::new(new_dont_visit);

                        SmallVisit::VisitedTwice
                    };

                    work_paths.push_back(Path2 {
                        small_visit, 
                        dont_visit,
                        pos: choice
                    });
                };

                // Visit once
                let mut new_dont_visit = HashSet::with_capacity(work_path.dont_visit.len() + 1);
                new_dont_visit.clone_from(&*work_path.dont_visit);
                new_dont_visit.insert(choice);
                let dont_visit = Rc::new(new_dont_visit);

                work_paths.push_back(Path2 {
                    small_visit: work_path.small_visit.clone(), 
                    dont_visit,
                    pos: choice
                });
            } else {
                work_paths.push_back(Path2 {
                    small_visit: work_path.small_visit.clone(), 
                    dont_visit: work_path.dont_visit.clone(),
                    pos: choice
                });
            }
        }
    }

    paths
}

fn build_tree(conns: &[Choice]) -> HashMap<String, Vec<String>> {
    let mut conn_choices: HashMap<String, Vec<String>> = HashMap::new();

    for conn in conns {
        if let Some(choices) = conn_choices.get_mut(&conn.from) {
            choices.push(conn.to.clone());
        } else {
            conn_choices.insert(conn.from.clone(), vec![conn.to.clone()]);
        }
    }

    conn_choices
}

#[inline]
fn lowercase_string(string: &str) -> bool {
    string.chars().all(char::is_lowercase)
}

struct Choice {
    from: String,
    to: String
}

type ParseResult = Vec<Choice>;

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

    // Create vector
    let mut conns = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let mut conn_iter = line.split('-');

            let choice1 = conn_iter.next().unwrap().to_string();
            let choice2 = conn_iter.next().unwrap().to_string();

            if choice2 != "start" {
                conns.push(Choice {
                    from: choice1.clone(),
                    to: choice2.clone(),
                });
            }

            if choice1 != "start" {
                conns.push(Choice {
                    from: choice2.clone(),
                    to: choice1.clone(),
                });
            }
        }
    }
   
    Ok(conns)
}

#[test]
fn test() {
    let paths = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    // Load connections
    let conns = load_buf(paths.as_bytes()).unwrap();

    // Build tree
    let tree = build_tree(&conns);

    let paths = count_paths1(&tree);
    assert_eq!(paths, 10);

    let paths = count_paths2(&tree);
    assert_eq!(paths, 36);
}
