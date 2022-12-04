#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use std::cmp::{max, min};
use std::{collections::HashSet, fs};

fn main() {
    let result: usize = fs::read_to_string("input/day4.txt")
        .expect("Failed to read input file")
        .lines()
        .filter(fully_contains)
        .count();

    println!("Result: {:?}", result);
}

fn fully_contains(line: &&str) -> bool {
    let (a, b) = line.split_once(',').unwrap();
    let a = parse_range(a);
    let b = parse_range(b);
    let outer = (min(a.0, b.0), max(a.1, b.1));

    a == outer || b == outer
}

fn parse_range(range: &str) -> (u32, u32) {
    let (a, b) = range.split_once('-').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}
