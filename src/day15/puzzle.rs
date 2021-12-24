use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::fmt::{Display, Formatter, write};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::result::Result as Res;
use std::str::{Chars, FromStr};
use std::string::ParseError;

const FILESTRING: &str = &"src/day15/lines.txt";
const GRID_SIZE: usize = 100;
const BIG_GRID_SIZE: usize = GRID_SIZE * 5;

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut grid: Grid = Grid { grid_size: GRID_SIZE, grid: [0u8; GRID_SIZE * GRID_SIZE] };
        for (i, line) in lines.enumerate() {
            for (j, cha) in line.unwrap().chars().enumerate() {
                grid[Point { x: j as isize, y: i as isize }] = char::to_digit(cha, 10).unwrap() as u8;
            }
        }
        let result: u32 = dijkstra(&grid, Point { x: 0, y: 0}, Point { x: (GRID_SIZE - 1) as isize, y: (GRID_SIZE - 1) as isize});
        println!("day15 puzzle 1: {}", result);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut grid: BigGrid = BigGrid { grid_size: BIG_GRID_SIZE, grid: [0u8; BIG_GRID_SIZE * BIG_GRID_SIZE] };
        for (i, line) in lines.enumerate() {
            for (j, cha) in line.unwrap().chars().enumerate() {
                for k in 0..5 {
                    for l in 0..5 {
                        let value: usize = char::to_digit(cha, 10).unwrap() as usize;
                        let new_value: usize = if (value + k + l) < 10 { (value + k + l) } else { ((value + k + l) % 10) + 1 };
                        grid[Point { x: (j + (GRID_SIZE) * k) as isize, y: (i + (GRID_SIZE) * l) as isize }] = new_value as u8;
                    }
                }
            }
        }
        let result: u32 = dijkstra_part2(&grid, Point { x: 0, y: 0}, Point { x: (BIG_GRID_SIZE - 1) as isize, y: (BIG_GRID_SIZE - 1) as isize});
        println!("day15 puzzle 2: {}", result);
    }
}

// No idea how to write that name >_<
fn dijkstra(grid: &Grid, start: Point, end: Point) -> u32 {
//     return the minimum risk level for the entire path from top left to bottom right.
//     astar is an informed search algorithm that goes from start to end by seeing the matrix as a weighted graph.
//     It maintains a tree of paths. At each iteration, it needs to determine which of its paths to extend.
//     To do so, it selects the path that minimizes f(p) = g(p) + h(p) where g is the cost to the Point p, and h is an heuristic
//     that attempts to guess the cost from Point p to the end.
    let mut discovered_nodes: HashSet<Point> = HashSet::new();

    let mut lowest_risk_to_nodes: HashMap<Point, u32> = HashMap::new();
    lowest_risk_to_nodes.insert(start, 0);

    let mut score: BinaryHeap<(Reverse<u32>, isize, isize)> = BinaryHeap::new();
    score.push((Reverse(0), start.x, start.y));

    while let Some((cost, x, y)) = score.pop() {
        let point: Point = Point { x, y };
        // Pop the most interesting node to visit, and if we haven't visited it yet
        if !discovered_nodes.contains(&point) {
            for neighbor in neighbors(&point, grid.grid_size) {
                if !discovered_nodes.contains(&neighbor) {
                    let value: u32 = (grid[neighbor] as u32) + cost.0;
                    let lowest_risk_to_neighbor: &mut u32 = lowest_risk_to_nodes.entry(neighbor).or_insert(u32::MAX);

                    if value < *lowest_risk_to_neighbor {
                        lowest_risk_to_nodes.insert(neighbor, value);
                        score.push((Reverse(value), neighbor.x, neighbor.y));
                    }
                }
            }
            discovered_nodes.insert(point);
            if point == end {
                break;
            }
        }
    }
    return *lowest_risk_to_nodes.get(&end).unwrap();
}

// No idea how to write that name >_<
fn dijkstra_part2(grid: &BigGrid, start: Point, end: Point) -> u32 {
//     return the minimum risk level for the entire path from top left to bottom right.
//     astar is an informed search algorithm that goes from start to end by seeing the matrix as a weighted graph.
//     It maintains a tree of paths. At each iteration, it needs to determine which of its paths to extend.
//     To do so, it selects the path that minimizes f(p) = g(p) + h(p) where g is the cost to the Point p, and h is an heuristic
//     that attempts to guess the cost from Point p to the end.
    let mut discovered_nodes: HashSet<Point> = HashSet::new();

    let mut lowest_risk_to_nodes: HashMap<Point, u32> = HashMap::new();
    lowest_risk_to_nodes.insert(start, 0);

    let mut score: BinaryHeap<(Reverse<u32>, isize, isize)> = BinaryHeap::new();
    score.push((Reverse(0), start.x, start.y));

    while let Some((cost, x, y)) = score.pop() {
        let point: Point = Point { x, y };
        // Pop the most interesting node to visit, and if we haven't visited it yet
        if !discovered_nodes.contains(&point) {
            for neighbor in neighbors(&point, grid.grid_size) {
                if !discovered_nodes.contains(&neighbor) {
                    let value: u32 = (grid[neighbor] as u32) + cost.0;
                    let lowest_risk_to_neighbor: &mut u32 = lowest_risk_to_nodes.entry(neighbor).or_insert(u32::MAX);

                    if value < *lowest_risk_to_neighbor {
                        lowest_risk_to_nodes.insert(neighbor, value);
                        score.push((Reverse(value), neighbor.x, neighbor.y));
                    }
                }
            }
            discovered_nodes.insert(point);
            if point == end {
                break;
            }
        }
    }
    return *lowest_risk_to_nodes.get(&end).unwrap();
}

fn neighbors(point: &Point, grid_size: usize) -> impl Iterator<Item = Point> {
    [
        // Here, putting the "+ 1" points first acts as an heuristic minimizing the remaining
        // distance to the bottom right corner
        Point { x: point.x, y: point.y + 1 },
        Point { x: point.x + 1, y: point.y },
        Point { x: point.x, y: point.y - 1 },
        Point { x: point.x - 1, y: point.y },
    ]
        .into_iter()
        .filter_map(move |p| {
            if p.x < 0 || p.x >= grid_size as isize || p.y < 0|| p.y >= grid_size as isize {
                None
            } else {
                Some(p)
            }
        })
}

struct Grid {
    grid_size: usize,
    grid: [u8; GRID_SIZE * GRID_SIZE],
}

struct BigGrid {
    grid_size: usize,
    grid: [u8; BIG_GRID_SIZE * BIG_GRID_SIZE],
}

impl Index<Point> for Grid {
    type Output = u8;
    fn index(&self, p: Point) -> &Self::Output {
        return &self.grid[(p.y as usize) * self.grid_size + (p.x as usize)];
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        return &mut self.grid[(p.y as usize) * self.grid_size + (p.x as usize)];
    }
}

impl Index<Point> for BigGrid {
    type Output = u8;
    fn index(&self, p: Point) -> &Self::Output {
        return &self.grid[(p.y as usize) * self.grid_size + (p.x as usize)];
    }
}

impl IndexMut<Point> for BigGrid {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        return &mut self.grid[(p.y as usize) * self.grid_size + (p.x as usize)];
    }
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Point {
    x: isize,
    y: isize
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point line {} column {}", self.y + 1, self.x + 1)
    }
}

impl Display for BigGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in 0..self.grid_size {
            let mut set: String = String::with_capacity((self.grid_size + 1) as usize);
            for j in 0..set.capacity() {
                set.push(' ');
            }
            for j in 0..self.grid_size {
                let value: u8 = *self.index(Point { x: j as isize, y: line as isize });
                set.replace_range(
                    set
                        .char_indices()
                        .nth(j)
                        .map(|(pos, ch)| (pos..pos + 1))
                        .unwrap()
                    , &*value.to_string());
            }
            write!(f, "{}\n", set);
        }
        write!(f,"\n")
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

