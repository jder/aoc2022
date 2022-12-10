#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use regex;
use std::cmp::{max, min};
use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input/day6.txt").expect("Failed to read input file");
    let (index, _) = content
        .chars()
        .tuple_windows()
        .enumerate()
        .find(|(_, (a, b, c, d))| a != b && a != c && a != d && b != c && b != d && c != d)
        .unwrap();

    println!("Result: {}", index + 4);
}
