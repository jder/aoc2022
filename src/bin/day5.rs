#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use regex;
use std::cmp::{max, min};
use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input/day5.txt").expect("Failed to read input file");
    let mut lines = content.lines();

    let mut stacks = vec![Vec::new(); 9];

    lines
        .take_while_ref(|line| line.len() > 0)
        .for_each(|line| {
            for (i, c) in line.chars().enumerate() {
                if c.is_ascii_uppercase() {
                    stacks[(i - 1) / 4].push(c);
                }
            }
        });

    for s in stacks.iter_mut() {
        *s = s.iter().cloned().rev().collect();
    }

    for l in lines {
        let Some(pieces) = regex::Regex::new(r"move (\d+) from (\d) to (\d)").unwrap().captures(l)
            else { continue };

        let count: usize = pieces[1].parse().unwrap();

        let src: usize = pieces[2].parse().unwrap();
        let dest: usize = pieces[3].parse().unwrap();

        for _ in 0..count {
            let c = stacks[src - 1].pop().unwrap();
            stacks[dest - 1].push(c);
        }
    }

    println!(
        "Result: {:?}",
        stacks.iter().map(|s| s.last().unwrap()).join("")
    );
}
