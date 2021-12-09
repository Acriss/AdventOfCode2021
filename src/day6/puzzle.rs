use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result, Lines};
use std::path::Path;

const FILESTRING: &str = &"src/day6/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut fishs: HashMap<u8, u32> = HashMap::new();
        for i in 0..9 as u8 {
            fishs.insert(i, 0);
        }
        if let Some(first) = lines.next() {
            match first {
                Err(_) => {}
                Ok(line) => {
                    line.split(',')
                        .map(|n| n.parse::<u8>().unwrap())
                        .for_each(|n| {
                            let count: &mut u32 = fishs.entry(n).or_insert(0);
                            *count += 1;
                        });
                }
            }
        }
        let days: u16 = 80;

        for _day in 0..days {
            let fish0: u32 = fishs[&0];
            let fish1: u32 = fishs[&1];
            let fish2: u32 = fishs[&2];
            let fish3: u32 = fishs[&3];
            let fish4: u32 = fishs[&4];
            let fish5: u32 = fishs[&5];
            let fish6: u32 = fishs[&6];
            let fish7: u32 = fishs[&7];
            let fish8: u32 = fishs[&8];

            fishs.insert(0, fish1);
            fishs.insert(1, fish2);
            fishs.insert(2, fish3);
            fishs.insert(3, fish4);
            fishs.insert(4, fish5);
            fishs.insert(5, fish6);
            fishs.insert(6, fish7 + fish0);
            fishs.insert(7, fish8);
            fishs.insert(8, fish0);
        }
        println!("day6 puzzle1: {}", fishs.values().sum::<u32>());
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let mut fishs: HashMap<u8, u64> = HashMap::new();
        for i in 0..9 as u8 {
            fishs.insert(i, 0);
        }
        if let Some(first) = lines.next() {
            match first {
                Err(_) => {}
                Ok(line) => {
                    line.split(',')
                        .map(|n| n.parse::<u8>().unwrap())
                        .for_each(|n| {
                            let count: &mut u64 = fishs.entry(n).or_insert(0);
                            *count += 1;
                        });
                }
            }
        }
        let days: u16 = 256;

        for _day in 0..days {
            let fish0: u64 = fishs[&0];
            let fish1: u64 = fishs[&1];
            let fish2: u64 = fishs[&2];
            let fish3: u64 = fishs[&3];
            let fish4: u64 = fishs[&4];
            let fish5: u64 = fishs[&5];
            let fish6: u64 = fishs[&6];
            let fish7: u64 = fishs[&7];
            let fish8: u64 = fishs[&8];

            fishs.insert(0, fish1);
            fishs.insert(1, fish2);
            fishs.insert(2, fish3);
            fishs.insert(3, fish4);
            fishs.insert(4, fish5);
            fishs.insert(5, fish6);
            fishs.insert(6, fish7 + fish0);
            fishs.insert(7, fish8);
            fishs.insert(8, fish0);
        }
        println!("day6 puzzle1: {}", fishs.values().sum::<u64>());
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
