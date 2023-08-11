use std::io::{BufRead, BufReader, Write};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the input file
    let tree = load_input("input12.txt")?;

    // Run parts
    part1(&tree);
    part2(&tree);

    // Write dot file (process with: dot -o output12.svg -T svg output12.dot)
    write_dot(&tree, "output12.dot")?;

    Ok(())
}

fn part1(tree: &Tree) {
    let paths = count_paths(tree, false);

    println!("Part 1: Number of paths visiting small caves once: {}", paths);
}

fn part2(tree: &Tree) {
    let paths = count_paths(tree, true);

    println!("Part 2: Number of paths visiting a small cave twice: {}", paths);
}

fn write_dot(tree: &Tree, file: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file);

    let mut file = File::create(path)?;

    writeln!(&mut file, "digraph caves {{")?;
    writeln!(&mut file, "\tconcentrate=true;")?;

    // Write start node
    writeln!(&mut file, "\tstart [color=red]")?;
    let stos: Vec<String> = tree.get("start").unwrap().iter().filter(|&s| s != "end").cloned().collect();
    writeln!(&mut file, "\tstart -> {{{}}};", &stos.join(" "))?;

    // Write other nodes
    for (from, tos) in tree {
        if from != "start" {
            let ftos: Vec<String> = tos.iter().filter(|&s| s != "end").cloned().collect();
            writeln!(&mut file, "\t{} -> {{{}}};", from, &ftos.join(" "))?;
        }
    }

    // Write end node
    writeln!(&mut file, "\tend [color=red]")?;
    for (from, tos) in tree {
        let ftos: Vec<String> = tos.iter().filter(|&s| s == "end").cloned().collect();
        
        if !ftos.is_empty() {
            writeln!(&mut file, "\t{} -> {{{}}};", from, &ftos.join(" "))?;
        }
    }

    writeln!(&mut file, "}}")?;

    Ok(())
}

struct CavePath<'a> {
    small_revisit: bool,
    visited: Rc<HashSet<&'a str>>,
    pos: &'a str
}

fn count_paths(tree: &Tree, allow_revisit: bool) -> usize {
    let mut paths: usize = 0;

    let is_small_cave = |name: &str| -> bool {
        name.chars().all(char::is_lowercase)
    };

    // Create work queue
    let mut work_paths: VecDeque<CavePath> = VecDeque::new();
    
    // Add initial work entry
    work_paths.push_back(CavePath {
        small_revisit: !allow_revisit,
        visited: Rc::new(HashSet::new()),
        pos: "start"
    });

    while let Some(work_path) = work_paths.pop_front() {
        // Get the tree entry
        let choices = tree.get(work_path.pos).unwrap();

        for choice in choices {
            if *choice == "end" {
                // Reached the end
                paths += 1;
                continue
            }

            // Variables for new work entry
            let mut small_revisit = work_path.small_revisit;
            let visited;

            if is_small_cave(choice) {
                // Small cave

                // Already visited?
                if work_path.visited.contains(&choice[..]) {
                    // Small cave has already been visited
                    if small_revisit {
                        continue
                    }

                    // Revisit this cave
                    small_revisit = true
                }

                // Build new visited hash set
                let mut new_visited_hashset = HashSet::with_capacity(work_path.visited.len() + 1);
                new_visited_hashset.clone_from(&*work_path.visited);
                new_visited_hashset.insert(choice);
                visited = Rc::new(new_visited_hashset);

            } else {
                // Large cave

                // Copy the existing visited hash set (by increasing the ref count)
                visited = work_path.visited.clone();

            }

            // Add new work unit
            work_paths.push_back(CavePath {
                small_revisit, 
                visited,
                pos: choice
            });
        }
    }

    paths
}

type Tree = HashMap<String, Vec<String>>;

fn load_input(file: &str) -> Result<Tree, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

fn load_buf(buf: &[u8]) -> Result<Tree, Box<dyn Error>> {
    // Create buf reader for the buffer
    let buf_reader = BufReader::new(buf);

    // Create vector
    let mut tree: Tree = HashMap::new();

    let add_tree = |tree: &mut Tree, from: &str, to: &str| {
        if to != "start" && from != "end" {
            if let Some(tree_ent) = tree.get_mut(from) {
                tree_ent.push(String::from(to));
            } else {
                tree.insert(String::from(from), vec![String::from(to)]);
            }
        }
    };

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let mut conn_iter = line.split('-');

            let choice1 = conn_iter.next().unwrap();
            let choice2 = conn_iter.next().unwrap();

            add_tree(&mut tree, choice1, choice2);
            add_tree(&mut tree, choice2, choice1);
        }
    }
   
    Ok(tree)
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
    let tree = load_buf(paths.as_bytes()).unwrap();

    let paths = count_paths(&tree, false);
    assert_eq!(paths, 10);

    let paths = count_paths(&tree, true);
    assert_eq!(paths, 36);
}
