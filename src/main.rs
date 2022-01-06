use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

fn main() {
    let now = Instant::now();
    // day1::puzzle::solve_puzzle1();
    // day1::puzzle::solve_puzzle2();
    // day2::puzzle::solve_puzzle1();
    // day2::puzzle::solve_puzzle2();
    // day3::puzzle::solve_puzzle1();
    // day3::puzzle::solve_puzzle2();
    // day4::puzzle::solve_puzzle1();
    // day4::puzzle::solve_puzzle2();
    // day5::puzzle::solve_puzzle1();
    // day5::puzzle::solve_puzzle2();
    // day6::puzzle::solve_puzzle1();
    // day6::puzzle::solve_puzzle2();
    // day7::puzzle::solve_puzzle1();
    // day7::puzzle::solve_puzzle2();
    // day8::puzzle::solve_puzzle1();
    // day8::puzzle::solve_puzzle2();
    // day9::puzzle::solve_puzzle1();
    // day9::puzzle::solve_puzzle2();
    // day10::puzzle::solve_puzzle1();
    // day10::puzzle::solve_puzzle2();
    // day11::puzzle::solve_puzzle1();
    // day11::puzzle::solve_puzzle2();
    // day12::puzzle::solve_puzzle1();
    // day12::puzzle::solve_puzzle2();
    // day13::puzzle::solve_puzzle1();
    // day13::puzzle::solve_puzzle2();
    // day14::puzzle::solve_puzzle1();
    // day14::puzzle::solve_puzzle2();
    // day15::puzzle::solve_puzzle1();
    // day15::puzzle::solve_puzzle2();
    // day16::puzzle::solve_puzzle1();
    // day16::puzzle::solve_puzzle2();
    day17::puzzle::solve_puzzle1();
    day17::puzzle::solve_puzzle2();
    let elapsed = now.elapsed();
    println!("total duration : {} ms", elapsed.as_secs_f64() * 1000.);
}
