use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::fmt::{Display, Formatter, write};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::result::Result as Res;
use std::str::{Chars, FromStr};
use std::string::ParseError;

const FILESTRING: &str = &"src/day14/lines.txt";

static ASCII: [char; 26] = [
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z',
];

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let (polymer, mut memoizer, growth_patterns): (Vec<char>, HashMap<MemoizationKey, [u64; 26]>, HashMap<Vec<char>, Vec<char>>) = initializations(lines);
        let iterations: usize = 10;
        let counts: [u64; 26] = recursively_solve_polymer(polymer, &mut memoizer, iterations, &growth_patterns);

        let max = counts.iter().max().unwrap();
        // Note that this filter will screw with testing with 1 or 0 iterations, as not all letters will appear,
        // and we are deliberately removing letters that never appear.
        let min = counts.iter().filter(|i| **i != 0).min().unwrap();
        println!("day14 puzzle 1: {}", max - min);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(lines) = read_lines(path) {
        let (polymer, mut memoizer, growth_patterns): (Vec<char>, HashMap<MemoizationKey, [u64; 26]>, HashMap<Vec<char>, Vec<char>>) = initializations(lines);
        let iterations: usize = 40;
        let counts: [u64; 26] = recursively_solve_polymer(polymer, &mut memoizer, iterations, &growth_patterns);

        let max = counts.iter().max().unwrap();
        // Note that this filter will screw with testing with 1 or 0 iterations, as not all letters will appear,
        // and we are deliberately removing letters that never appear.
        let min = counts.iter().filter(|i| **i != 0).min().unwrap();
        println!("day14 puzzle 1: {}", max - min);
    }
}

fn recursively_solve_polymer(
    polymer: Vec<char>,
    mut memoizer: &mut HashMap<MemoizationKey, [u64; 26]>,
    iterations : usize,
    growth_patterns: &HashMap<Vec<char>, Vec<char>>)
    -> [u64; 26] {
    let first_key: MemoizationKey = MemoizationKey { polymer: polymer.to_owned(), step: iterations };
    return match memoizer.get(&first_key) {
        None => {
            // If we are at the last step, simply count the letters
            if iterations == 0 {
                let mut counts: [u64; 26] = [0; 26];
                for ch in polymer.iter() {
                    increment(&mut counts, *ch);
                }
                return counts;
            } else {
                let mut tmp_counts: Vec<u64> = Vec::from([0; 26]);
                // This is not the last step: cut the polymer by pairs, solve recursively for each 3-letter polymers we get for one step smaller, and concatenate the letter counts.
                // Be careful with that concatenation, as the middle letters appear twice,once as the left char of a pair, once as its right char.
                for window in polymer.clone().windows(2) {
                    let left: char = window[0];
                    let right: char = window[1];
                    let step: usize = iterations - 1;
                    let expanded_polymer: Vec<char> = growth_patterns.get(&vec![left, right]).unwrap().to_owned();
                    let key = MemoizationKey { polymer: expanded_polymer, step };
                    let window_count: [u64; 26] = recursively_solve_polymer(
                        key.polymer.to_owned(),
                        memoizer,
                        key.step.to_owned(),
                        growth_patterns);
                    match memoizer.get(&key) {
                        None => {
                            memoizer.insert(key.to_owned(), window_count.to_owned());
                        }
                        Some(_) => {}
                    }

                    // Summing the counts of the windows before with this window.
                    // We must remember that this is counting twice the character that is common to
                    // the two windows, and we must remove the counts of all characters except first and last
                    // from the finalized tmp_counts
                    tmp_counts = tmp_counts
                        .iter()
                        .zip(window_count.into_iter())
                        .map(|(a, b)| a + b)
                        .collect::<Vec<u64>>();
                }
                // Removing the chars that were counted twice, one time for each pair they are part of.
                let mut polymer_resolved: [u64; 26] = demo(tmp_counts);
                for i in 1..polymer.len() - 1 {
                    decrement(&mut polymer_resolved, polymer[i])
                }
                memoizer.insert(first_key.to_owned(), polymer_resolved.to_owned());
                polymer_resolved
            }
        }
        Some(result) => { *result }
    }

}

fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

// Counts will be in alphabetical order : B, C, H, N
fn initializations(mut lines: Lines<BufReader<File>>) -> (Vec<char>, HashMap<MemoizationKey, [u64; 26]>, HashMap<Vec<char>, Vec<char>>) {
    let mut blank_reached: bool = false;
    let mut polymer: Vec<char> = Vec::new();
    let mut growth_patterns: HashMap<Vec<char>, Vec<char>> = HashMap::new();
    let mut keys: HashMap<MemoizationKey, [u64; 26]> = HashMap::new();
    while let Some(line) = lines.next() {
        let result: String = line.unwrap();
        if result.is_empty() {
            blank_reached = true;
            continue;
        }
        if !blank_reached {
            for character in result.chars() {
                polymer.push(character);
            }
        } else {
            let coords: Vec<&str> = result.split_whitespace().collect();
            let key: MemoizationKey = coords[0].parse::<MemoizationKey>().unwrap();
            let final_char: char = coords[2].chars().next().unwrap();
            let mut counts: [u64; 26] = [0; 26];
            let left_letter: char = key.polymer[0];
            let right_letter: char = key.polymer[1];
            increment(&mut counts, left_letter);
            increment(&mut counts, right_letter);
            keys.insert(key.to_owned(), counts);
            growth_patterns.insert(vec![left_letter, right_letter], vec![left_letter, final_char ,right_letter]);
        }
    }
    return (polymer, keys, growth_patterns);
}

fn increment(counts: &mut [u64; 26], letter: char) {
    let index: usize = ASCII.iter().position(|ch| *ch == letter).unwrap();
    counts[index] = counts[index] + 1;
}

fn decrement(counts: &mut [u64; 26], letter: char) {
    let index: usize = ASCII.iter().position(|ch| *ch == letter).unwrap();
    counts[index] = counts[index] - 1;
}

#[derive(Debug,Clone,Hash,Eq,PartialEq)]
struct MemoizationKey {
    polymer: Vec<char>,
    step: usize
}

impl Display for MemoizationKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for i in 0..self.polymer.len() {
            string.push(self.polymer[i]);
        }
        write!(f, "Polymer {} step {}", string, self.step)
    }
}

impl FromStr for MemoizationKey {
    type Err = ParseError;

    fn from_str(s: &str) -> Res<Self, Self::Err> {
        let mut chars: Chars = s.chars();
        let left: char = chars.next().unwrap();
        let right: char = chars.next().unwrap();
        let polymer: Vec<char> = vec![left, right];
        let step: usize = 0;
        Ok(MemoizationKey { polymer, step })
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

