use std::fmt::{Display, Formatter, write};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::ops::{Index, IndexMut, RangeInclusive};
use std::path::Path;
use std::str::{Chars, FromStr};

const FILESTRING: &str = &"src/day17/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let target: Target = parse_target(lines.next().unwrap().unwrap().as_str());

        // because of the x velocity change, solving the x(time) gives xmax = dx(dx+1)/2, and Xinitial=0
        // thus, dx must be solution of dx2 + dx - 2xmin = 0
        let dxmin: isize = (-1. + (1. + 8. * target.xmin as f32).sqrt() / 2.).ceil() as isize;
        let dxrange: RangeInclusive<isize> = dxmin..=target.xmax;
        let dymax: isize = target.ymin.abs().max(target.ymax.abs());
        let result: isize = dxrange
            .flat_map(|dx| (-dymax..=dymax).map(move |dy| (dx, dy)))
            .filter_map(|(dx, dy)| match Probe::new(dx, dy).shoot(&target) {
                Hit {
                    hit_target: false,
                    ymax: _,
                } => None,
                Hit {
                    hit_target: true,
                    ymax,
                } => Some(ymax),
            })
            .max()
            .unwrap_or(0);
        println!("day17 puzzle 1: {}", result);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let target: Target = parse_target(lines.next().unwrap().unwrap().as_str());

        // because of the x velocity change, solving the x(time) gives xmax = dx(dx+1)/2, and Xinitial=0
        // thus, dx must be solution of dx2 + dx - 2xmin = 0
        let dxmin: isize = (-1. + (1. + 8. * target.xmin as f32).sqrt() / 2.).ceil() as isize;
        let dxrange: RangeInclusive<isize> = dxmin..=target.xmax;
        let dymax: isize = target.ymin.abs().max(target.ymax.abs());
        let result: usize = dxrange
            .flat_map(|dx| (-dymax..=dymax).map(move |dy| (dx, dy)))
            .filter(|(dx, dy)| Probe::new(*dx, *dy).shoot(&target).hit_target)
            .count();
        println!("day17 puzzle 2: {}", result);
    }
}

#[derive(Debug)]
struct Probe {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct Hit {
    hit_target: bool,
    ymax: isize,
}

impl Probe {
    fn new(dx: isize, dy: isize) -> Self {
        Self { x: 0, y: 0, dx, dy }
    }

    fn shoot(&mut self, target: &Target) -> Hit {
        let mut ymax = self.y;
        while !self.in_target(&target) && !self.too_far(&target) {
            self.step();
            ymax = ymax.max(self.y);
        }

        Hit {
            hit_target: self.in_target(&target),
            ymax,
        }
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dx -= self.dx.signum();
        self.dy -= 1;
    }

    fn in_target(&self, target: &Target) -> bool {
        self.x >= target.xmin
            && self.x <= target.xmax
            && self.y >= target.ymin
            && self.y <= target.ymax
    }

    fn too_far(&self, target: &Target) -> bool {
        (self.dy <= 0 && self.y < target.ymin) || self.x > target.xmax
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Target {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

fn parse_target(target_str: &str) -> Target {
    let (_, coordinates): (&str, &str) = target_str.split_once(": ").unwrap();
    let (x_range, y_range): (&str, &str) = coordinates.split_once(", ").unwrap();
    let (xmin_str, xmax_str) = x_range.split_once("..").unwrap();
    let (ymin_str, ymax_str) = y_range.split_once("..").unwrap();
    let xmin: isize = xmin_str.split_at(2).1.parse().unwrap();
    let xmax: isize = xmax_str.parse().unwrap();
    let ymin: isize = ymin_str.split_at(2).1.parse().unwrap();
    let ymax: isize = ymax_str.parse().unwrap();
    Target { xmin, xmax, ymin, ymax }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

