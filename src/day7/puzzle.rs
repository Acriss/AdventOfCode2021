use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result, Lines};
use std::path::Path;

const FILESTRING: &str = &"src/day7/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut crabs: HashMap<u32, u32> = HashMap::new();
        if let Some(first) = lines.next() {
            match first {
                Err(_) => {}
                Ok(line) => {
                    line.split(',')
                        .map(|n| n.parse::<u32>().unwrap())
                        .for_each(|n| {
                            let count: &mut u32 = crabs.entry(n).or_insert(0);
                            *count += 1;
                        });
                }
            }
        }
        let min_pos: u32 = crabs.keys().min().unwrap().to_owned();
        let max_pos: u32 = crabs.keys().max().unwrap().to_owned();

        let mut min_total_fuel: u32 = u32::MAX;
        for position in min_pos..(max_pos + 1) {
            let mut total_fuel_for_position: u32 = 0;
            for crab_position in crabs.borrow() {
                let fuel_per_crab: u32 = i32::abs((position as i32) - (*crab_position.0 as i32)) as u32;
                let fuel_for_crabs_at_position: u32 = *crab_position.1 * fuel_per_crab;
                total_fuel_for_position += fuel_for_crabs_at_position;
            }
            if total_fuel_for_position < min_total_fuel {
                min_total_fuel = total_fuel_for_position;
            }
        }
        println!("day7 puzzle1: {}", min_total_fuel);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut crabs: HashMap<u32, u32> = HashMap::new();
        if let Some(first) = lines.next() {
            match first {
                Err(_) => {}
                Ok(line) => {
                    line.split(',')
                        .map(|n| n.parse::<u32>().unwrap())
                        .for_each(|n| {
                            let count: &mut u32 = crabs.entry(n).or_insert(0);
                            *count += 1;
                        });
                }
            }
        }
        let min_pos: u32 = crabs.keys().min().unwrap().to_owned();
        let max_pos: u32 = crabs.keys().max().unwrap().to_owned();

        let mut min_total_fuel: u32 = u32::MAX;
        for position in min_pos..(max_pos + 1) {
            let mut total_fuel_for_position: u32 = 0;
            for crab_position in crabs.borrow() {
                let step: u32 = i32::abs((position as i32) - (*crab_position.0 as i32)) as u32;
                let fuel_per_crab: u32 = step * (step + 1) / 2;
                let fuel_for_crabs_at_position: u32 = *crab_position.1 * fuel_per_crab;
                total_fuel_for_position += fuel_for_crabs_at_position;
            }
            if total_fuel_for_position < min_total_fuel {
                min_total_fuel = total_fuel_for_position;
            }
        }
        println!("day7 puzzle1: {}", min_total_fuel);
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
