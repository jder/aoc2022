#[allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use std::fs;

fn main() {
    let max: u32 = fs::read_to_string("input/day1.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|a| a.parse().unwrap_or_default())
        .coalesce(|a: u32, b| if b == 0 { Err((a, b)) } else { Ok(a + b) })
        .sorted()
        .rev()
        .take(3)
        .sum::<u32>();
    println!("Max: {:?}", max);
}
