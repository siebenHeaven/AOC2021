mod day1;
mod day2;
mod day3;
mod day4;

use std::env;
use std::io::{self, BufRead};
use std::time::Instant;
fn main() {
    let day: u32 = env::args().collect::<Vec<_>>()[1].parse().unwrap();
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|lines| lines.unwrap())
        .collect();

    let start = Instant::now();
    match day {
        1 => day1::solve(&input),
        2 => day2::solve(&input),
        3 => day3::solve(&input),
        4 => day4::solve(&input),
        _ => todo!(),
    }
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
