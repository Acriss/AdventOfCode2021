use std::cmp::{Ordering, Reverse};
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

const FILESTRING: &str = &"src/day16/lines.txt";

pub fn solve_puzzle() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let line: String = lines.next().unwrap().unwrap().to_owned();
        let mut transmission: Transmission = Transmission::new(line.as_str());
        println!("day16 puzzle 1: {}", transmission.parse_packet().unwrap().value);
    }
}

struct Transmission<'a> {
    buffer: Buffer<'a>,
}

impl<'a> Transmission<'a> {
    fn new<'b: 'a>(input: &'b str) -> Self {
        Self {
            buffer: Buffer::new(input),
        }
    }

    fn parse_header(&mut self) -> Option<(u8, u8)> {
        let version: u8 = self.buffer.get_n_bits(3)? as u8;
        let type_id: u8 = self.buffer.get_n_bits(3)? as u8;
        Some((version, type_id))
    }

    fn parse_literal(&mut self) -> Option<u64> {
        let mut v: u64 = 0;
        while self.buffer.get_n_bits(1)? == 1 {
            v <<= 4;
            v += self.buffer.get_n_bits(4)? as u64;
        }
        v <<= 4;
        v += self.buffer.get_n_bits(4)? as u64;
        Some(v)
    }

    fn parse_binary_operator(&mut self, f: impl FnOnce(u64, u64) -> u64) -> Option<u64> {
        let len_id = self.buffer.get_n_bits(1)?;
        if len_id == 0 {
            let _bits_len = self.buffer.get_n_bits(15)?;
        } else {
            let _nb_packet = self.buffer.get_n_bits(11)?;
        }
        let x = self.parse_packet()?.value;
        let y = self.parse_packet()?.value;
        Some(f(x, y))
    }

    fn parse_reduce_operator(
        &mut self,
        mut acc: u64,
        mut f: impl FnMut(u64, u64) -> u64,
    ) -> Option<u64> {
        let len_id = self.buffer.get_n_bits(1)?;
        if len_id == 0 {
            let bits_len = self.buffer.get_n_bits(15)?;
            let max = self.buffer.current_position * 4 - (self.buffer.length as usize) + bits_len as usize;
            while self.buffer.current_position * 4 - (self.buffer.length as usize) < max {
                let v = self.parse_packet()?.value;
                acc = f(acc, v);
            }
        } else {
            let nb_packet = self.buffer.get_n_bits(11)?;
            for _ in 0..nb_packet {
                let v = self.parse_packet()?.value;
                acc = f(acc, v);
            }
        }
        Some(acc)
    }

    fn parse_packet(&mut self) -> Option<Packet> {
        let (version, type_id): (u8, u8) = self.parse_header()?;
        let v = match type_id {
            0 => self.parse_reduce_operator(0, |acc, v| acc + v),
            1 => self.parse_reduce_operator(1, |acc, v| acc * v),
            2 => self.parse_reduce_operator(u64::MAX, |acc, v| acc.min(v)),
            3 => self.parse_reduce_operator(0, |acc, v| acc.max(v)),
            4 => self.parse_literal(),
            5 => self.parse_binary_operator(|a, b| if a > b { 1 } else { 0 }),
            6 => self.parse_binary_operator(|a, b| if a < b { 1 } else { 0 }),
            7 => self.parse_binary_operator(|a, b| if a == b { 1 } else { 0 }),
            _ => panic!(),
        }?;
        Some(Packet {
            version,
            type_id,
            value: v,
        })
    }
}

impl<'a> Iterator for Transmission<'a> {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_packet()
    }
}

struct Buffer<'a>  {
    length: u8,// Can be mutated
    data: u32, // Can be mutated
    current_position: usize, // Current position in bytes array
    bytes: &'a [u8]
}

impl<'a> Buffer<'a> {
    fn new<'l: 'a>(input: &'l str) -> Self {
        Self {
            length: 0,
            data: 0,
            current_position: 0,
            bytes:input.as_bytes(),
        }
    }

    fn get_n_bits(&mut self, n:u8) -> Option<u32> {
        while n > self.length {
            // Add 4n bits into the buffer, at most
            self.add_bits()?;
        }
        let remaining: u8 = self.length - n;
        let value: u32 = self.data & (u32::MAX << remaining); // Shift the mask
        self.data &= (1 << remaining) - 1;
        self.length -= n;
        Some(value >> remaining)
    }

    // Add 4 bits into the buffer
    fn add_bits(&mut self) -> Option<()> {
        if self.current_position  >= self.bytes.len() {
            return None;
        }
        let current_bytes: u8 = self.bytes[self.current_position];
        self.current_position += 1;
        let v: u8 = if current_bytes <= b'9' { current_bytes - b'0' } else { 10 + current_bytes - b'A' };
        self.data <<= 4;
        self.data |= v as u32;
        self.length += 4;
        Some(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    value: u64,
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

