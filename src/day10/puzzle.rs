use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

const FILESTRING: &str = &"src/day10/lines.txt";

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let parentheses: Vec<String> = lines
            .map(|line| line.unwrap())
            .map(|line| line.to_owned())
            .collect();
        let mut scores: HashMap<char, u32> = HashMap::new();
        scores.insert(')', 3);
        scores.insert('>', 25137);
        scores.insert(']', 57);
        scores.insert('}', 1197);
        let mut mapping: HashMap<char, char> = HashMap::new();
        mapping.insert('(', ')');
        mapping.insert('[', ']');
        mapping.insert('{', '}');
        mapping.insert('<', '>');
        let mut score: u32 = 0;
        for parenthesis in parentheses {
            let mut queue: VecDeque<char> = VecDeque::new();
            'inner: for char in parenthesis.chars() {
                if mapping.contains_key(&char) {
                    queue.push_front(char);
                } else {
                    let associated_char = queue.pop_front();
                    match associated_char {
                        None => { //     The given char was the last in the queue and is closing. That's illegal.
                        }
                        Some(c) => {
                            if char == *mapping.get(&c).unwrap() {
                                // Great, the two are closing each other, we can ignore them
                            } else {
                                // They are not matching. Let's stop and get points
                                score += scores.get(&char).unwrap();
                                break 'inner;
                            }
                        }
                    }
                }
            }
        }
        println!("day10 puzzle1: {}", score);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let parentheses: Vec<String> = lines
            .map(|line| line.unwrap())
            .map(|line| line.to_owned())
            .collect();
        let mut scores: HashMap<char, u64> = HashMap::new();
        scores.insert(')', 1);
        scores.insert('>', 4);
        scores.insert(']', 2);
        scores.insert('}', 3);
        let mut mapping: HashMap<char, char> = HashMap::new();
        mapping.insert('(', ')');
        mapping.insert('[', ']');
        mapping.insert('{', '}');
        mapping.insert('<', '>');
        let mut score: Vec<u64> = Vec::new();
        'outer: for parenthesis in parentheses {
            let mut queue: VecDeque<char> = VecDeque::new();
            for char in parenthesis.chars() {
                if mapping.contains_key(&char) {
                    queue.push_front(char);
                } else {
                    let associated_char = queue.pop_front();
                    match associated_char {
                        None => { //     The given char was the last in the queue and is closing. That's illegal.
                        }
                        Some(c) => {
                            if char == *mapping.get(&c).unwrap() {
                                // Great, the two are closing each other, we can ignore them
                            } else {
                                // They are not matching. Let's stop and ignore this corrupted line
                                continue 'outer;
                            }
                        }
                    }
                }
            }
            let mut partial_score: u64 = 0;
            while !queue.is_empty() {
                let char: char = queue.pop_front().unwrap();
                // Let's assume we only have opening chars in the queue. It should be.
                let closing_matching: &char = mapping.get(&char).unwrap();
                partial_score = (5 * partial_score) + scores.get(closing_matching).unwrap();
            }
            score.push(partial_score);
        }
        score.sort();
        println!("day10 puzzle2: {}", score.get((score.len() - 1) / 2).unwrap());
    }
}



fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

