use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

const FILESTRING: &str = &"src/day4/bingos.txt";

pub fn solve_puzzle1() {
    // Hopefully there are less elements than 2^16, else we should change to u32
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut numbers: Vec<u32> = Vec::new();
        if let Some(first) = lines.next() {
            numbers = parse_first_line(first);
        }
        let (bingos, mut counters) = parse_bingos(lines);
        let mut winning_bingo: usize = usize::MAX;
        let mut winning_number: u32 = u32::MAX;
        'outer: for number in &numbers {
            for (bingo_number, bingo) in bingos.iter().enumerate() {
                let index: Option<usize> = bingo.iter().position(|&r| r == *number);
                match index {
                    None => {}
                    Some(i) => {
                        let counter = counters.get_mut(bingo_number).unwrap();
                        let row = i / 5;
                        let column = 5 + (i % 5);

                        *counter.get_mut(row).unwrap() += 1;
                        *counter.get_mut(column).unwrap() += 1;
                        let size_row = *counter.get(row).unwrap();
                        let size_col = *counter.get(column).unwrap();
                        if size_row == 5 || size_col == 5 {
                            winning_bingo = bingo_number;
                            winning_number = *number;
                            break 'outer;
                        }
                    }
                }
            }
        }
        let bingo = bingos.get(winning_bingo).unwrap();
        let mut sum: u32 = bingo.iter().sum();
        for number in numbers {
            let index: Option<usize> = bingo.iter().position(|&r| r == number);
            match index {
                None => {}
                Some(i) => { sum -= bingo.get(i).unwrap();}
            }
            if number == winning_number {
                break;
            }
        }
        println!("day4 puzzle1: {}", sum * winning_number);
    }
}

pub fn solve_puzzle2() {
    // Hopefully there are less elements than 2^16, else we should change to u32
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut numbers: Vec<u32> = Vec::new();
        if let Some(first) = lines.next() {
            numbers = parse_first_line(first);
        }
        let (bingos, mut counters) = parse_bingos(lines);
        let mut winning_bingos: Vec<usize> = Vec::new();
        let mut winning_bingo: usize = usize::MAX;
        let mut winning_number: u32 = u32::MAX;
        for number in &numbers {
            for (bingo_number, bingo) in bingos.iter().enumerate() {
                let has_bingo_already_won = winning_bingos.iter().position(|&r| r == bingo_number);
                match has_bingo_already_won {
                    None => {
                        let index: Option<usize> = bingo.iter().position(|&r| r == *number);
                        match index {
                            None => {}
                            Some(i) => {
                                let counter = counters.get_mut(bingo_number).unwrap();
                                let row = i / 5;
                                let column = 5 + (i % 5);

                                *counter.get_mut(row).unwrap() += 1;
                                *counter.get_mut(column).unwrap() += 1;
                                let size_row = *counter.get(row).unwrap();
                                let size_col = *counter.get(column).unwrap();
                                if size_row == 5 || size_col == 5 {
                                    winning_bingos.push(bingo_number);
                                    winning_bingo = bingo_number;
                                    winning_number = *number;
                                }
                            }
                        }
                    }
                    Some(_) => {}
                }
            }
        }
        let bingo = bingos.get(winning_bingo).unwrap();
        let mut sum: u32 = bingo.iter().sum();
        for number in &numbers {
            let index: Option<usize> = bingo.iter().position(|&r| r == *number);
            match index {
                None => {}
                Some(i) => { sum -= bingo.get(i).unwrap();}
            }
            if *number == winning_number {
                break;
            }
        }
        println!("day4 puzzle2: {}", sum * winning_number);
    }
}


fn parse_first_line(line: Result<String>) -> Vec<u32> {
    return match line {
        Ok(f) => f
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect(),
        Err(_) => { Vec::new() }
    };
}

fn parse_bingos(mut lines: Lines<BufReader<File>>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let bingo_size: usize = 5;
    let mut bingos: Vec<Vec<u32>> = Vec::new();
    let mut counters: Vec<Vec<u32>> = Vec::new();
    let mut current_bingo: Vec<u32> = Vec::new();
    while let Some(l) = lines.next() {
        let previous_size: usize = current_bingo.len();
        let mut b: Vec<u32>;
        match l {
            Ok(f) => b = f
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect(),
            Err(_) => { b = Vec::new() }
        };
        current_bingo.append(&mut b);
        if current_bingo.len() == previous_size {
            // line was blank. Get to new bingo.
            bingos.push(current_bingo.clone());
            counters.push(vec![0; 2 * bingo_size]);
            current_bingo = Vec::new();
        }
    }
    return (bingos, counters);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
