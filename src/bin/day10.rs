#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::num;
use std::path::{Path, PathBuf};
use std::{collections::HashSet, fs};

fn main() {
    let mut cycle = 1;
    let mut x = 1;

    let samples = [20, 60, 100, 140, 180, 220];

    let mut total = 0;

    for line in fs::read_to_string("input/day10.txt")
        .expect("Failed to read input file")
        .lines()
    {
        if line == "noop" {
            total += end_cycle(&mut cycle, x, &samples);
            continue;
        } else if let Some(r) = regex::Regex::new(r"addx (-?\d+)").unwrap().captures(&line) {
            total += end_cycle(&mut cycle, x, &samples);
            total += end_cycle(&mut cycle, x, &samples);
            x += r[1].parse::<i32>().unwrap();
        } else {
            panic!("Invalid instruction: {}", line);
        }
    }

    println!("{}", total);
}

fn end_cycle(cycle: &mut i32, x: i32, samples: &[i32]) -> i32 {
    let result = if samples.contains(cycle) {
        println!("{}: {}", cycle, x);
        x * *cycle
    } else {
        0
    };

    *cycle += 1;
    result
}
