use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

const FILESTRING: &str = &"src/day9/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let mut map: HashMap<Point,isize> = HashMap::new();
        for (y, line) in lines.enumerate() {
            for (x, c) in line.unwrap().chars().enumerate(){
                map.insert(Point { x: x as isize, y: y as isize}, String::from(c).parse::<isize>().unwrap());
            }
        }
        let mut basins: Vec<Point> = Vec::new();
        let mut total = 0;
        for square in map.keys() {
            let mut found = true;
            let height = map.get(&square).unwrap();
            for n in get_neighbours(&square) {
                if height >= map.get(&n).unwrap_or(&isize::MAX) {
                    found = false;
                    break;
                }
            }
            if found {
                basins.push(*square);
                total += height +1
            }
        }
        println!("day9 puzzle1: {}", total);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let mut map: HashMap<Point,isize> = HashMap::new();
        for (y, line) in lines.enumerate() {
            for (x, c) in line.unwrap().chars().enumerate(){
                map.insert(Point { x: x as isize, y: y as isize}, String::from(c).parse::<isize>().unwrap());
            }
        }
        let mut points: Vec<Point> = Vec::new();
        for square in map.keys() {
            let mut found: bool = true;
            let height: &isize = map.get(&square).unwrap();
            for n in get_neighbours(&square) {
                if height >= map.get(&n).unwrap_or(&isize::MAX) {
                    found = false;
                    break;
                }
            }
            if found {
                points.push(*square);
            }
        }
        println!("day9 puzzle1: {}", find_basins(points, map));
    }
}

// Will remove points from the map !
fn find_basins(points: Vec<Point>, mut map: HashMap<Point, isize>) -> isize{
    let mut basin_sizes: HashMap<Point, isize> = HashMap::new();
    for startpoint in points {
        let mut considered: VecDeque<Point> = VecDeque::new();
        considered.push_back(startpoint);
        let mut size: isize = 0;
        while considered.len() > 0 {
            let c: Point = considered.pop_front().unwrap();
            if map.contains_key(&c){
                size +=1;
                let height: isize = *map.get(&c).unwrap();
                map.remove(&c);
                for n in get_neighbours(&c) {
                    let nh: isize = *map.get(&n).unwrap_or(&isize::MAX);
                    if nh > height && nh != 9{
                        considered.push_back(n);
                    }
                }
            }
        }
        basin_sizes.insert(startpoint, size);
    }
    let mut sizes: Vec<&isize> = basin_sizes.values().collect::<Vec<&isize>>();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Point {
    x: isize,
    y: isize
}

fn get_neighbours(c: &Point) -> Vec<Point> {
    vec![Point {x: c.x-1, y: c.y}, Point {x: c.x+1, y: c.y}, Point {x: c.x, y: c.y+1}, Point {x: c.x, y: c.y-1} ]
}
