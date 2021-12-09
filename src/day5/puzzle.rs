use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash};
use std::io::{self, BufRead, BufReader, Lines};
use std::io::Result as Res;
use std::num::ParseIntError;
use std::path::Path;
use std::result::Result;
use std::str::FromStr;

const FILESTRING: &str = &"src/day5/lines.txt";

pub fn solve_puzzle1() {
    // Hopefully there are less elements than 2^16, else we should change to u32
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let filtered_vents: Vec<Vent> = lines
            .map(|line| line.unwrap())
            .map(|n| n.parse::<Vent>().unwrap())
            .filter(|s| s.p1.x == s.p2.x || s.p1.y == s.p2.y)
            .collect();
        let mut counters: HashMap<Point, u32> = HashMap::new();
        for filtered_vent in filtered_vents {
            if filtered_vent.p1.x == filtered_vent.p2.x {
                let x: u32 = filtered_vent.p1.x;
                let max_y: u32 = max(filtered_vent.p1.y, filtered_vent.p2.y);
                let min_y: u32 = min(filtered_vent.p1.y, filtered_vent.p2.y);
                for i in min_y..(max_y + 1) {
                    let counter: &mut u32 = counters.entry(Point { x, y: i }).or_insert(0);
                    *counter += 1;
                }
            } else if filtered_vent.p1.y == filtered_vent.p2.y {
                let y: u32 = filtered_vent.p1.y;
                let max_x: u32 = max(filtered_vent.p1.x, filtered_vent.p2.x);
                let min_x: u32 = min(filtered_vent.p1.x, filtered_vent.p2.x);
                for i in min_x..(max_x + 1) {
                    let counter: &mut u32 = counters.entry(Point { x: i, y }).or_insert(0);
                    *counter += 1;
                }
            }
        }
        let mut count: u32 = 0;
        for counter in counters.values() {
             if *counter > 1 {
                 count += 1;
             }
        }
        println!("day5 puzzle1: {}", count);
    }
}

pub fn solve_puzzle2() {
    // Hopefully there are less elements than 2^16, else we should change to u32
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let vents: Vec<Vent> = lines
            .map(|line| line.unwrap())
            .map(|n| n.parse::<Vent>().unwrap())
            .collect();
        let mut counters: HashMap<Point, u32> = HashMap::new();
        for vent in vents {
            if vent.p1.x == vent.p2.x {
                let x: u32 = vent.p1.x;
                let max_y: u32 = max(vent.p1.y, vent.p2.y);
                let min_y: u32 = min(vent.p1.y, vent.p2.y);
                for i in min_y..(max_y + 1) {
                    let counter: &mut u32 = counters.entry(Point { x, y: i }).or_insert(0);
                    *counter += 1;
                }
            } else if vent.p1.y == vent.p2.y {
                let y: u32 = vent.p1.y;
                let max_x: u32 = max(vent.p1.x, vent.p2.x);
                let min_x: u32 = min(vent.p1.x, vent.p2.x);
                for i in min_x..(max_x + 1) {
                    let counter: &mut u32 = counters.entry(Point { x: i, y }).or_insert(0);
                    *counter += 1;
                }
            } else {
                let max_y: u32 = max(vent.p1.y, vent.p2.y);
                let min_y: u32 = min(vent.p1.y, vent.p2.y);
                let max_x: u32 = max(vent.p1.x, vent.p2.x);
                let min_x: u32 = min(vent.p1.x, vent.p2.x);
                if max_y - min_y == max_x - min_x {
                    // diagonale
                    // Need to find which type of diagonale
                    if vent.p1.x == min_x && vent.p1.y == min_y {
                        for i in 0..(max_x - min_x + 1) {
                            let counter: &mut u32 = counters.entry(Point { x: i + min_x, y: i + min_y }).or_insert(0);
                            *counter += 1;
                        }
                    } else if vent.p2.x == min_x && vent.p2.y == min_y {
                        for i in 0..(max_x - min_x + 1) {
                            let counter: &mut u32 = counters.entry(Point { x: i + min_x, y: i + min_y }).or_insert(0);
                            *counter += 1;
                        }
                    } else if vent.p1.x == min_x && vent.p2.y == min_y {
                        for i in 0..(max_x - min_x + 1) {
                            let counter: &mut u32 = counters.entry(Point { x: i + min_x, y: max_y - i }).or_insert(0);
                            *counter += 1;
                        }
                    } else if vent.p2.x == min_x && vent.p1.y == min_y {
                        for i in 0..(max_x - min_x + 1) {
                            let counter: &mut u32 = counters.entry(Point { x: i + min_x, y: max_y - i }).or_insert(0);
                            *counter += 1;
                        }
                    }
                }
            }
        }
        let mut count: u32 = 0;
        for counter in counters.values() {
             if *counter > 1 {
                 count += 1;
             }
        }
        println!("day5 puzzle2: {}", count);
    }
}

fn read_lines<P>(filename: P) -> Res<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Hash)]
struct Point {
    x: u32,
    y: u32
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        let x_fromstr: u32 = coords[0].parse::<u32>()?;
        let y_fromstr: u32 = coords[1].parse::<u32>()?;
        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}
struct Vent {
    p1: Point,
    p2: Point
}
impl FromStr for Vent {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: String = s.replace(" -> ", ",");
        let coords: Vec<&str> = string.split(',').collect();
        let x1_fromstr: u32 = coords[0].parse::<u32>()?;
        let y1_fromstr: u32 = coords[1].parse::<u32>()?;
        let x2_fromstr: u32 = coords[2].parse::<u32>()?;
        let y2_fromstr: u32 = coords[3].parse::<u32>()?;
        Ok(Vent { p1: Point {x: x1_fromstr, y: y1_fromstr}, p2: Point {x: x2_fromstr, y: y2_fromstr}})
    }
}
