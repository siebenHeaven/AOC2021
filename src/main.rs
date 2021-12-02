mod day1;
mod day2;

use std::env;
use std::io::{self, BufRead};
fn main() {
    let day: u32 = env::args().collect::<Vec<_>>()[1].parse().unwrap();
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|lines| lines.unwrap())
        .collect();

    match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        _ => todo!(),
    }
}
