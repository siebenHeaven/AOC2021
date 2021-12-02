mod day1;

use std::io::{self, BufRead};
fn main() {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|lines| lines.unwrap())
        .collect();

    day1::solve(input)
}
