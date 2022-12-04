#[allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let result: u32 = fs::read_to_string("input/day3.txt")
        .expect("Failed to read input file")
        .lines()
        .map(score_rucksack)
        .sum();

    println!("Result: {:?}", result);
}

fn score_rucksack(line: &str) -> u32 {
    let (items_first, items_second) = line.split_at(line.len() / 2);
    let items_first: HashSet<char> = items_first.chars().collect();
    let common = items_second
        .chars()
        .find(|c| items_first.contains(&c))
        .unwrap();
    score_for(common)
}

fn score_for(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid character: {}", c),
    }
}
