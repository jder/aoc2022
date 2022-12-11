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
    let mut output = [['?'; 40]; 6];

    let mut cycle = 1;
    let mut x = 1;

    for line in fs::read_to_string("input/day10.txt")
        .expect("Failed to read input file")
        .lines()
    {
        if line == "noop" {
            end_cycle(&mut cycle, x, &mut output);
            continue;
        } else if let Some(r) = regex::Regex::new(r"addx (-?\d+)").unwrap().captures(&line) {
            end_cycle(&mut cycle, x, &mut output);
            end_cycle(&mut cycle, x, &mut output);
            x += r[1].parse::<i32>().unwrap();
        } else {
            panic!("Invalid instruction: {}", line);
        }
    }

    println!(
        "{}",
        output
            .iter()
            .map(|x| x.iter().collect::<String>())
            .join("\n")
    );
}

fn end_cycle(cycle: &mut i32, x: i32, output: &mut [[char; 40]; 6]) {
    let char_out = if (x - (*cycle - 1) % 40).abs() <= 1 {
        '#'
    } else {
        '.'
    };
    output[((*cycle - 1) / 40) as usize][((*cycle - 1) % 40) as usize] = char_out;
    *cycle += 1;
}
