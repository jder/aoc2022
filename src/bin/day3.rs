#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let result: usize = fs::read_to_string("input/day3.txt")
        .expect("Failed to read input file")
        .lines()
        .chunks(3)
        .into_iter()
        .map(find_common)
        .sum();

    println!("Result: {:?}", result);
}

fn find_common<'a>(i: impl Iterator<Item = &'a str>) -> usize {
    let mut counts = [0usize; 26 * 2 + 1];
    for line in i {
        for c in line.chars().unique() {
            counts[score_for(c)] += 1;
        }
    }

    counts
        .iter()
        .enumerate()
        .find_map(|(i, c)| if *c == 3 { Some(i) } else { None })
        .unwrap()
}

fn score_rucksack(line: &str) -> usize {
    let (items_first, items_second) = line.split_at(line.len() / 2);
    let items_first: HashSet<char> = items_first.chars().collect();
    let common = items_second
        .chars()
        .find(|c| items_first.contains(&c))
        .unwrap();
    score_for(common)
}

fn score_for(c: char) -> usize {
    match c {
        'a'..='z' => (c as usize) - ('a' as usize) + 1,
        'A'..='Z' => (c as usize) - ('A' as usize) + 27,
        _ => panic!("Invalid character: {}", c),
    }
}
