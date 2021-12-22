use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::result::Result as Res;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::str::FromStr;

const FILESTRING: &str = &"src/day13/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let (points, folds) = points_and_folds(lines);
        let mut points_after_first_fold: HashSet<Point> = HashSet::new();
        println!("{} {}", folds[0].horizontal, folds[0].position);
        for point in points {
            let new_point: Option<Point> = fold_point(point, folds[0]);
            match new_point {
                None => {continue}
                Some(p) => {
                    if points_after_first_fold.contains(&p) {
                        continue;
                    }
                    points_after_first_fold.insert(p);
                }
            }
        }
        println!("day13 puzzle 1: {}", points_after_first_fold.len());
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let (mut points, folds) = points_and_folds(lines);
        for fold in folds {
            let mut points_after_fold: HashSet<Point> = HashSet::new();
            for point in points {
                let new_point: Option<Point> = fold_point(point, fold);
                match new_point {
                    None => {continue}
                    Some(p) => {
                        if points_after_fold.contains(&p) {
                            continue;
                        }
                        points_after_fold.insert(p);
                    }
                }
            }
            points = points_after_fold;
        }
        // display
        let max_x: u32 = points.iter().map(|p| p.x).max().unwrap();
        let max_y: u32 = points.iter().map(|p| p.y).max().unwrap();
        println!("day13 puzzle 2: ");
        for i in 0..max_y + 1 {
            let mut set: String = String::with_capacity((max_x + 1) as usize);
            for j in 0..set.capacity() {
                set.push(' ');
            }
            for point in &points {
                if point.y == i {
                    set.replace_range(
                        set
                            .char_indices()
                            .nth(point.x as usize)
                            .map(|(pos, ch)| (pos..pos + 1))
                            .unwrap()
                        ,"#");
                }
            }
            println!("{}", set);
        }
    }
}

fn fold_point(point: Point, fold: Fold) -> Option<Point> {
    return if fold.horizontal {
        fold_point_horizontally(point, fold.position)
    } else {
        fold_point_vertically(point, fold.position)
    }
}

fn fold_point_horizontally(point: Point, axis: u32) -> Option<Point> {
    return if point.y < axis {
        Some(point)
    } else if point.y == axis {
        None
    } else {
        Some(Point { x: point.x.clone(), y: 2 * axis - point.y })
    }
}

fn fold_point_vertically(point: Point, axis: u32) -> Option<Point> {
    return if point.x < axis {
        Some(point)
    } else if point.x == axis {
        None
    } else {
        Some(Point { x: 2 * axis - point.x , y: point.y.clone() })
    }
}

fn points_and_folds(mut lines: Lines<BufReader<File>>) -> (HashSet<Point>, Vec<Fold>) {
    let mut fold_reached: bool = false;
    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();
    while let Some(line) = lines.next() {
        let result: String = line.unwrap();
        if result.is_empty() {
            fold_reached = true;
            continue;
        }
        if !fold_reached {
            points.insert(result.parse::<Point>().unwrap());
        } else {
            folds.push(result.parse::<Fold>().unwrap());
        }
    }
    return (points, folds);
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Point {
    x: u32,
    y: u32
}
#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Fold {
    position: u32,
    horizontal: bool
}

impl FromStr for Fold {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Res<Self, Self::Err> {
        let coords: Vec<&str> = s.split('=').collect();
        let pos_fromstr: u32 = coords[1].parse::<u32>()?;
        let hor_fromstr: bool = if (coords[0]).split_whitespace().last().unwrap().eq("y") { true } else { false };
        Ok(Fold { position: pos_fromstr, horizontal: hor_fromstr })
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Res<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        let x_fromstr: u32 = coords[0].parse::<u32>()?;
        let y_fromstr: u32 = coords[1].parse::<u32>()?;
        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}


fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

