#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::num;
use std::path::{Path, PathBuf};
use std::{collections::HashSet, fs};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn new(dir: &str) -> Direction {
        match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction: {}", dir),
        }
    }
}

fn main() {
    let mut head: (i32, i32) = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for line in fs::read_to_string("input/day9.txt")
        .expect("Failed to read input file")
        .lines()
    {
        let (dir, count) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let dir = Direction::new(dir);
        let count = count.parse::<i32>().unwrap();

        for _ in 0..count {
            match dir {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }

            let tail_to_head = (head.0 - tail.0, head.1 - tail.1);
            match tail_to_head {
                (x, y) if x.abs() <= 1 && y.abs() <= 1 => continue,
                (x, 0) => tail.0 += x.signum(),
                (0, y) => tail.1 += y.signum(),
                (x, y) => {
                    tail.0 += x.signum();
                    tail.1 += y.signum();
                }
            }

            visited.insert(tail);
        }
    }

    println!("Result: {}", visited.len());
}
