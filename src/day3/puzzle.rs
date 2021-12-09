use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

const FILESTRING: &str = &"src/day3/binary.txt";

pub fn solve_puzzle1() {
    // Hopefully there are less elements than 2^16, else we should change to u32
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        const LINE_SIZE: usize = 12;
        let mut counts: [u16; LINE_SIZE] = [0; LINE_SIZE];
        let mut line_count: u16 = 0;
        while let Some(line) = lines.next() {
            if let Ok(words) = line {
                for (index, char) in words.chars().enumerate() {
                    if char == '1' {
                        counts[index] += 1;
                    }
                }
                line_count += 1;
            }
        }
        let mut gamma: u64 = 0;
        for i in 0..LINE_SIZE {
            if counts[i] > (line_count / 2) {
                // There are more ones than zeroes: gamma gains, at bit i, a 1.
                // Writing this as a sum of powers of 2.
                gamma += u64::pow(2, (LINE_SIZE - 1 - i) as u32)
            }
        }
        let mask_to_zero: u64 = (1 << LINE_SIZE) - 1;
        println!("day3 puzzle1: {}", gamma * (mask_to_zero ^ gamma));
    }
}

pub fn solve_puzzle2() {
    // Hopefully there are less elements than 2^16, else we should change to u32
    let path: &Path = Path::new(FILESTRING);
    let mut strings: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let l: String = line.unwrap();
            strings.push(l);
        }
    }
    let binaries: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    println!(
        "day3 puzzle2: {}",
        keep_most_common(binaries.clone(), 0)
            * keep_least_common(binaries, 0));
}

fn keep_most_common(numbers: Vec<&str>, idx: usize) -> u64 {
    return if numbers.len() > 1 {
        let mut ones: Vec<&str> = Vec::new();
        let mut zeroes: Vec<&str> = Vec::new();
        for n in numbers {
            if n.chars().nth(idx).unwrap() == '1' {
                ones.push(&n);
            } else {
                zeroes.push(&n);
            }
        }
        if ones.len() >= zeroes.len() {
            keep_most_common(ones, idx + 1)
        } else {
            keep_most_common(zeroes, idx + 1)
        }
    } else {
        u64::from_str_radix(numbers[0], 2).unwrap_or_default()
    }
}

fn keep_least_common(numbers: Vec<&str>, idx: usize) -> u64 {
    return if numbers.len() > 1 {
        let mut ones: Vec<&str> = Vec::new();
        let mut zeroes: Vec<&str> = Vec::new();
        for n in numbers {
            if n.chars().nth(idx).unwrap() == '1' {
                ones.push(n);
            } else {
                zeroes.push(n);
            }
        }
        if ones.len() >= zeroes.len() {
            keep_least_common(zeroes, idx + 1)
        } else {
            keep_least_common(ones, idx + 1)
        }
    } else {
        u64::from_str_radix(numbers[0], 2).unwrap_or_default()
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
