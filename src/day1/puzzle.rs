use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, Result, Lines, BufReader};
use std::slice::Windows;

const FILESTRING: &str = &"src/day1/depths.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let depths: Vec<u32> = get_depths(lines);
        let mut previous_depth: Option<u32> = None;
        let mut count: u32 = 0;
        for depth in depths {
            match previous_depth {
                Some(d) => {
                    if depth > d {
                        count += 1;
                    }
                    previous_depth = Some(depth);
                },
                None => previous_depth = Some(depth),
            }
        }
        println!("day1 puzzle1: {}", count);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let depths: Vec<u32> = get_depths(lines);
        let mut windows: Windows<u32> = depths.windows(3);
        let mut previous_cumulative_depth: Option<u32> = None;
        let mut count: u32 = 0;
        while let Some(next_window) = windows.next() {
            match previous_cumulative_depth {
                None => previous_cumulative_depth = Some(next_window.iter().sum::<u32>()),
                Some(old_depth) => {
                    let new_depth: u32 = next_window.iter().sum();
                    if new_depth > old_depth {
                        count += 1;
                    }
                    previous_cumulative_depth = Some(new_depth);
                }
            }
        }
        println!("day1 puzzle2: {}", count);
    }
}

fn get_depths(lines: Lines<BufReader<File>>) -> Vec<u32> {
    return lines.map(|x| x.unwrap().parse::<u32>().expect("Invalid line"))
        .collect();

}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
