use std::collections::{HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result, Lines};
use std::result::Result as Res;
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

const FILESTRING: &str = &"src/day8/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let puzzles: Vec<Puzzle> = lines
            .map(|line| line.unwrap())
            .map(|n| n.parse::<Puzzle>().unwrap())
            .collect();
        println!("day8 puzzle1: {}", puzzles.iter().flat_map(|puzzle| puzzle.fours.iter())
            .map(|result| result.len())
            .filter(|size| *size == 2 || *size == 3 || *size == 4 || *size == 7)
            .count());
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let mut sum = 0;
            let puzzles: Vec<Puzzle> = lines
                .map(|line| line.unwrap())
                .map(|n| n.parse::<Puzzle>().unwrap())
                .collect();
        for puzzle in puzzles {
            let (mut one, mut four) = (HashSet::new(), HashSet::new());
            for pattern in puzzle.tens {
                match pattern.as_bytes().len() {
                    2 => one.extend(pattern.as_bytes()),
                    4 => four.extend(pattern.as_bytes()),
                    _ => {}
                }
            }

            sum += puzzle.fours.iter()
                .map(|number| {
                    let nb = HashSet::from_iter(number.as_bytes().iter().cloned());
                    match (
                        nb.len(),
                        nb.intersection(&one).count(),
                        nb.intersection(&four).count(),
                    ) {
                        (2, _, _) => 1,
                        (4, _, _) => 4,
                        (3, _, _) => 7,
                        (7, _, _) => 8,
                        (5, 2, _) => 3,
                        (5, 1, 3) => 5,
                        (5, 1, 2) => 2,
                        (6, 1, _) => 6,
                        (6, 2, 4) => 9,
                        (6, 2, 3) => 0,
                        _ => panic!("invalid number: {:?}", number),
                    }
                })
                .fold(0, |acc, x| 10 * acc + x);
        }

        println!("day8 puzzle1: {}", sum);
    }
}

struct Puzzle {
    tens: Vec<String>,
    fours: Vec<String>
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Res<Self, Self::Err> {
        let string: String = s.replace(" | ", "|");
        let coords: Vec<&str> = string.split('|').collect();
        let tens = coords.get(0).unwrap().split(' ').to_owned().map(|n| String::from(n)).collect();
        let fours = coords.get(1).unwrap().split(' ').to_owned().map(|n| String::from(n)).collect();
        Ok(Puzzle {tens, fours})
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
