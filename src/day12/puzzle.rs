use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::ops::{Index, IndexMut};
use std::path::Path;

const FILESTRING: &str = &"src/day12/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let mut octos: Octopuses = Octopuses::new();
        for (i, line) in lines.enumerate() {
            for (j, cha) in line.unwrap().chars().enumerate() {
                octos[Point { x: j, y: i}] = char::to_digit(cha, 10).unwrap() as u8;
            }
        }
        let mut count: u32 = 0;
        for _i in 0..ITER_COUNT {
            count += octos.step();
        }
        println!("day11 puzzle 1: {}", count);
    }
}



pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let mut octos: Octopuses = Octopuses::new();
        for (i, line) in lines.enumerate() {
            for (j, cha) in line.unwrap().chars().enumerate() {
                octos[Point { x: j, y: i}] = char::to_digit(cha, 10).unwrap() as u8;
            }
        }
        let mut count: usize = 0;
        let mut step: usize = 0;
        while count != (GRID_SIZE * GRID_SIZE) {
            octos.step();
            step += 1;
            count = octos.blinks.blinks.iter().filter(|b| **b).count();
        }
        println!("day11 puzzle 2: {}", step);
    }
}

const GRID_SIZE: usize = 10;
const ITER_COUNT: usize = 100;

struct Octopuses {
    grid: [u8; GRID_SIZE * GRID_SIZE],
    blinks: Blinks
}

struct Blinks {
    blinks: [bool; GRID_SIZE * GRID_SIZE]
}

impl Display for Octopuses {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                write!(f, "{}", *self.index(Point { x: j, y: i }));
            }
            write!(f, "\n");
        }
        write!(f, "\n")
    }
}

impl Index<Point> for Blinks {
    type Output = bool;
    fn index(&self, p: Point) -> &Self::Output {
        return &self.blinks[p.y * GRID_SIZE + p.x];
    }
}

impl IndexMut<Point> for Blinks {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        return &mut self.blinks[p.y * GRID_SIZE + p.x];
    }
}

impl Index<Point> for Octopuses {
    type Output = u8;
    fn index(&self, p: Point) -> &Self::Output {
        return &self.grid[p.y * GRID_SIZE + p.x];
    }
}

impl IndexMut<Point> for Octopuses {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        return &mut self.grid[p.y * GRID_SIZE + p.x];
    }
}

impl Blinks {
    fn new() -> Self {
        Self {
            blinks: [false; GRID_SIZE * GRID_SIZE],
        }
    }
}

impl Octopuses {
    fn new() -> Self {
        Self {
            grid: [0u8; GRID_SIZE * GRID_SIZE],
            blinks: Blinks::new(),
        }
    }

    fn step(&mut self) -> u32 {
        let mut blink_count: u32 = 0;
        self.blinks = Blinks::new();
        let mut blinkers: Vec<Point> = Vec::new();
        // Traversal top to bottom, left to right.
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let point: Point = Point { x: j, y: i };
                // Reaching a point. We add one and if it's blinking, compute that impact
                self.add_one(point);
               if *self.index(point) > 9 {
                   blinkers.push(point);
               }
            }
        }
        while let(Some(point)) = blinkers.pop() {
            if !self.has_blinked(point) {
                blink_count += 1;
                self.compute_blinking_impact(point, &mut blinkers);
            }
        }
        return blink_count;
    }

    fn set_has_blink(&mut self, point: Point) {
        *self.blinks.index_mut(point) = true;
        *self.index_mut(point) = 0;
    }

    fn has_blinked(&mut self, point: Point) -> bool {
        return *self.blinks.index(point);
    }

    fn compute_blinking_impact(&mut self, point: Point, blinkers: &mut Vec<Point>)  {
        self.set_has_blink(point);

        // * add 1 to the forward neighbors.
        // * compute its impact to backward-neighbors.
        let neighbors: Vec<Point> = get_neighbors(&point);
        for neighbor in neighbors {
            if !self.has_blinked(neighbor) {
                self.add_one(neighbor);
                if *self.index(neighbor) > 9 {
                    blinkers.push(neighbor);
                }
            }
        }
    }

    fn add_one(&mut self, point: Point) {
        let octo: &mut u8 = self.index_mut(point);
        *octo += 1;
    }
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Point {
    x: usize,
    y: usize
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point line {} column {}", self.y + 1, self.x + 1)
    }
}

fn get_neighbors(p: &Point) -> Vec<Point> {
    return if p.y == 0 {
        if p.x == 0 {
            vec![
                Point { x: p.x + 1, y: p.y },
                Point { x: p.x, y: p.y + 1 },
                Point { x: p.x + 1, y: p.y + 1 }
            ]
        } else if p.x == GRID_SIZE - 1 {
            // p is on first line, last column
            vec![
                Point { x: p.x, y: p.y + 1 },
                Point { x: p.x - 1, y: p.y },
                Point { x: p.x - 1, y: p.y + 1 }
            ]
        } else {
            vec![
                Point { x: p.x + 1, y: p.y },
                Point { x: p.x, y: p.y + 1 },
                Point { x: p.x + 1, y: p.y + 1 },
                Point { x: p.x - 1, y: p.y },
                Point { x: p.x - 1, y: p.y + 1 }
            ]
        }
    } else if p.y == GRID_SIZE - 1 {
        if p.x == GRID_SIZE - 1 {
            // Last column, last line
            vec![
                Point { x: p.x, y: p.y - 1 },
                Point { x: p.x - 1, y: p.y },
                Point { x: p.x - 1, y: p.y - 1 },
            ]
        } else if p.x == 0 {
            // First column, last line
            vec![
                Point { x: p.x + 1, y: p.y },
                Point { x: p.x + 1, y: p.y - 1 },
                Point { x: p.x, y: p.y - 1 },
            ]
        } else {
            // last line
            vec![
                Point { x: p.x + 1, y: p.y },
                Point { x: p.x + 1, y: p.y - 1 },
                Point { x: p.x, y: p.y - 1 },
                Point { x: p.x - 1, y: p.y },
                Point { x: p.x - 1, y: p.y - 1 },
            ]
        }
    } else {
        if p.x == GRID_SIZE - 1 {
            vec![
                Point { x: p.x, y: p.y - 1 },
                Point { x: p.x - 1, y: p.y - 1 },
                Point { x: p.x - 1, y: p.y },
                Point { x: p.x, y: p.y + 1 },
                Point { x: p.x - 1, y: p.y + 1 },
            ]
        } else if p.x == 0 {
            vec![
                Point { x: p.x, y: p.y - 1 },
                Point { x: p.x + 1, y: p.y - 1 },
                Point { x: p.x + 1, y: p.y },
                Point { x: p.x, y: p.y + 1 },
                Point { x: p.x + 1, y: p.y + 1 },
            ]
        } else {
            vec![
                Point { x: p.x - 1, y: p.y - 1 },
                Point { x: p.x, y: p.y - 1 },
                Point { x: p.x + 1, y: p.y - 1 },
                Point { x: p.x - 1, y: p.y },
                Point { x: p.x + 1, y: p.y },
                Point { x: p.x - 1, y: p.y + 1 },
                Point { x: p.x, y: p.y + 1 },
                Point { x: p.x + 1, y: p.y + 1 },
            ]
        }
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

