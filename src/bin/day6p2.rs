#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use regex;
use std::cmp::{max, min};
use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input/day6.txt").expect("Failed to read input file");
    let bytes = content.as_bytes();
    'outer: for index in 0..bytes.len() {
        let run_length = 14;
        let mut found = [false; 256];
        for i in 0..run_length {
            let c = bytes[index + i];
            if found[c as usize] {
                continue 'outer;
            }
            found[c as usize] = true;
        }

        println!("Found at: {}", index + run_length);
        break;
    }
}
