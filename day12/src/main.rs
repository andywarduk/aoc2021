use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
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
    let paths = count_paths(tree, false);

    println!("Part 1: Number of paths visiting small caves once: {}", paths);
}

fn part2(tree: &HashMap<String, Vec<String>>) {
    let paths = count_paths(tree, true);

    println!("Part 2: Number of paths visiting a small cave twice: {}", paths);
}

struct Path<'a> {
    small_revisit: bool,
    visited: Rc<HashSet<&'a str>>,
    pos: &'a str
}

fn count_paths(tree: &HashMap<String, Vec<String>>, allow_revisit: bool) -> usize {
    let mut paths: usize = 0;

    // Create work queue
    let mut work_paths: VecDeque<Path> = VecDeque::new();
    
    // Add initial work entry
    work_paths.push_back(Path {
        small_revisit: !allow_revisit,
        visited: Rc::new(HashSet::new()),
        pos: "start"
    });

    while let Some(work_path) = work_paths.pop_front() {
        // Get the tree entry
        let choices = tree.get(work_path.pos).unwrap();

        for choice in choices {
            if choice == "end" {
                // Reached the end
                paths += 1;
                continue
            }

            // Variables for new work entry
            let mut revisit = work_path.small_revisit;
            let new_visited;

            if lowercase_string(choice) {
                // Small cave
                if work_path.visited.get(&choice[..]).is_some() {
                    // Small cave has already been visited
                    if revisit {
                        continue
                    }

                    // Revisit this cave
                    revisit = true
                }

                // Build new visited hash set
                let mut new_visited_hashset = HashSet::with_capacity(work_path.visited.len() + 1);
                new_visited_hashset.clone_from(&*work_path.visited);
                new_visited_hashset.insert(choice);
                new_visited = Rc::new(new_visited_hashset);

            } else {
                // Copy the existing visited hash set (by increasing the ref count)
                new_visited = work_path.visited.clone();

            }

            // Add new work unit
            work_paths.push_back(Path {
                small_revisit: revisit, 
                visited: new_visited,
                pos: choice
            });
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

fn load_input(file: &str) -> Result<Vec<Choice>, Box<dyn Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Load from the mmapped vile
    load_buf(mmap.as_ref())
}

fn load_buf(buf: &[u8]) -> Result<Vec<Choice>, Box<dyn Error>> {
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

    let paths = count_paths(&tree, false);
    assert_eq!(paths, 10);

    let paths = count_paths(&tree, true);
    assert_eq!(paths, 36);
}
