use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;
use std::str::SplitWhitespace;

const FILESTRING: &str = &"src/day2/operations.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut depth: i32 = 0;
        let mut horizontal: i32 = 0;
        while let Some(line) = lines.next() {
            if let Ok(words) = line {
                let mut dual: SplitWhitespace = words.split_whitespace();
                let verb: &str = dual.next().unwrap_or_default();
                let value: i32 = dual.next().unwrap_or_default().parse::<i32>().unwrap_or_default();
                match verb {
                    "forward" => horizontal += value,
                    "down" => depth += value,
                    "up" => depth -= value,
                    _ => {}
                }
            }
        }
        println!("day2 puzzle1: {}", depth * horizontal);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut depth: i32 = 0;
        let mut horizontal: i32 = 0;
        let mut aim: i32 = 0;
        while let Some(line) = lines.next() {
            if let Ok(words) = line {
                let mut dual: SplitWhitespace = words.split_whitespace();
                let verb: &str = dual.next().unwrap_or_default();
                let value: i32 = dual.next().unwrap_or_default().parse::<i32>().unwrap_or_default();
                match verb {
                    "forward" => {
                        horizontal += value;
                        depth += aim * value;
                    },
                    "down" => aim += value,
                    "up" => aim -= value,
                    _ => {}
                }
            }
        }
        println!("day2 puzzle2: {}", depth * horizontal);
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
