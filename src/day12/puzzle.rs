use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::ops::{Add, Index, IndexMut};
use std::path::Path;

const FILESTRING: &str = &"src/day12/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let mut graph: Graph = Graph::new();
        for line in lines {
            let s: String = line.unwrap();
            graph.ingest_caves_and_tunnels(s);
        }
        let mut visited_caves: HashSet<usize> = HashSet::new();
        let mut score: u32 = 0;
        score += graph.visit_neighbors("start", visited_caves);
        println!("day12 puzzle 1: {}", score)
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let mut graph: Graph = Graph::new();
        for line in lines {
            let s: String = line.unwrap();
            graph.ingest_caves_and_tunnels(s);
        }
        let mut visited_caves: HashSet<usize> = HashSet::new();
        let mut score: u32 = 0;
        score += graph.visit_neighbors_with_chosen_small_cave(
            "start",
            visited_caves,
            None);
        println!("day12 puzzle 2: {}", score)
    }
}

struct Graph {
    // both vectors must be aligned.
    cave_names: Vec<String>,
    caves: Vec<Cave>,
}

impl Graph {
    fn new() -> Self {
        Self {
            cave_names: vec![],
            caves: vec![]
        }
    }

    fn add_new_cave(&mut self, cave_name: &str) -> usize {
        self.cave_names.push(cave_name.to_string());
        self.caves.push(Cave::new(cave_name.to_string()));
        return self.cave_names.len() - 1;
    }

    fn ingest_caves_and_tunnels(&mut self, line: String) {
        let (cave1, cave2): (&str, &str) = line.split_once('-').unwrap();
        // Side effect to add the new cave.
        let index_cave_1: usize = self.get_or_create_cave_index(cave1);
        let index_cave_2: usize = self.get_or_create_cave_index(cave2);
        self.create_tunnels(index_cave_1, index_cave_2);
    }

    fn get_or_create_cave_index(&mut self, cave_name: &str) -> usize {
        return self.cave_names.iter().position(|name| name == cave_name)
            .unwrap_or_else(|| self.add_new_cave(cave_name));
    }

    fn get_cave_index(&self, cave_name: &str) -> usize {
        return self.cave_names.iter().position(|name| name == cave_name)
            .unwrap();
    }

    fn create_tunnels(&mut self, index_cave_1: usize, index_cave_2: usize) {
        let mut cave: &mut Cave = self.caves.get_mut(index_cave_1).unwrap();
        cave.tunnel_to(self.cave_names.get(index_cave_2).unwrap());

        cave = self.caves.get_mut(index_cave_2).unwrap();
        cave.tunnel_to(self.cave_names.get(index_cave_1).unwrap());
    }

    fn visit_neighbors(&mut self, cave_name: &str, mut already_visited: HashSet<usize>) -> u32 {
        if cave_name == "end" {
            return 1;
        }
        let mut count = 0;
        let current_cave_index: usize = self.get_or_create_cave_index(cave_name);
        let current_cave: &Cave = self.caves.get(current_cave_index).unwrap();
        if current_cave.small {
            already_visited.insert(current_cave_index);
        }
        for neighbor in current_cave.neighbors.clone().into_iter() {
            let neighbor_index: usize = self.get_or_create_cave_index(&neighbor);
            if !already_visited.contains(&neighbor_index) {
                // The clone here is very important, to ensure that at one step of visitation, we don't
                // prevent the next paths from being found because of a wrongly computed set.
                // Here when getting start,A,b,A,c,A,end, we will go back to start,A
                // but must still be able to see the other paths that will contain c afterward like start,A,c,A,end
                count += self.visit_neighbors(&neighbor, already_visited.clone());
            }
        }
        return count;
    }

    fn visit_neighbors_with_chosen_small_cave(
        &self,
        cave_name: &str,
        mut already_visited: HashSet<usize>,
        chosen_small_cave: Option<usize>) -> u32 {
        let mut count = 0;
        let current_cave_index: usize = self.get_cave_index(cave_name);
        let mut current_cave: &Cave = &self.caves[current_cave_index];
        let start: &str = "start";
        let end: &str = "end";
        if start.ne(cave_name) && current_cave.small {
            if chosen_small_cave.is_none() {
                for neighbor in current_cave.neighbors.clone().into_iter() {
                    let neighbor_index = &self.get_cave_index(&neighbor);
                    if end.ne(&neighbor)
                        && !already_visited.contains(neighbor_index) {
                        count += self.visit_neighbors_with_chosen_small_cave(
                            &neighbor,
                            already_visited.clone(),
                        Some(current_cave_index))
                    }
                }
            }
        }
        if current_cave.small {
            already_visited.insert(current_cave_index);
        }
        for neighbor in current_cave.neighbors.clone().into_iter() {
            let neighbor_index = &self.get_cave_index(&neighbor);
            if end.eq(&neighbor) {
                if chosen_small_cave.is_none() || already_visited.contains(&chosen_small_cave.unwrap()) {
                    count += 1;
                }
            } else if !already_visited.contains(neighbor_index) {
                count += self.visit_neighbors_with_chosen_small_cave(
                    &neighbor,
                    already_visited.clone(),
                    chosen_small_cave);
            }

        }

        return count;
    }
}

#[derive(Hash,Eq,PartialEq)]
struct Cave {
    name: String,
    small: bool,
    neighbors: Vec<String>,
}

impl Cave {
    fn new(cave_name: String) -> Self {
        Self {
            name: cave_name.to_owned(),
            small: cave_name == cave_name.to_lowercase(),
            neighbors: vec![]
        }
    }
    fn tunnel_to(&mut self, new_neighbor: &String) {
        self.neighbors.push(new_neighbor.to_owned());
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

