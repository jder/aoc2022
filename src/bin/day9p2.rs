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
    let mut knots = vec![(0i32, 0i32); 10];

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
                Direction::Up => knots[0].1 += 1,
                Direction::Down => knots[0].1 -= 1,
                Direction::Left => knots[0].0 -= 1,
                Direction::Right => knots[0].0 += 1,
            }

            for i in 1..knots.len() {
                let head = knots[i - 1];
                let tail = knots[i];

                let tail_to_head = (head.0 - tail.0, head.1 - tail.1);
                match tail_to_head {
                    (x, y) if x.abs() <= 1 && y.abs() <= 1 => continue,
                    (x, 0) => knots[i].0 += x.signum(),
                    (0, y) => knots[i].1 += y.signum(),
                    (x, y) => {
                        knots[i].0 += x.signum();
                        knots[i].1 += y.signum();
                    }
                }
            }

            visited.insert(knots[knots.len() - 1]);
        }
    }

    println!("Result: {}", visited.len());
}
