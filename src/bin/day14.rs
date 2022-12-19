#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use num;
use peg;
use regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{collections::HashSet, fs};

fn main() {
    let points_re = regex::Regex::new(r"(\d+),(\d+)").unwrap();

    let lines: Vec<Vec<(isize, isize)>> = fs::read_to_string("input/day14.txt")
        .unwrap()
        .lines()
        .map(|line| {
            points_re
                .captures_iter(line)
                .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
                .collect()
        })
        .collect();

    let max_coord = *lines
        .iter()
        .map(|line| line.iter().max_by_key(|(x, y)| max(x, y)).unwrap())
        .max_by_key(|(x, y)| max(x, y))
        .unwrap();
    let max_coord = max(max_coord.0, max_coord.1);

    let mut filled = vec![vec![false; max_coord as usize + 1]; max_coord as usize + 1];
    for line in lines {
        paint_line(&mut filled, &line);
    }

    'outer: for grain in 0.. {
        let mut location = (500, 0);
        loop {
            if location.1 == max_coord {
                println!("Grain {} reached the bottom", grain);
                break 'outer;
            }
            if !filled[location.0 as usize][location.1 as usize + 1] {
                location.1 += 1;
                continue;
            } else if !filled[location.0 as usize - 1][location.1 as usize + 1] {
                location.0 -= 1;
                location.1 += 1;
                continue;
            } else if !filled[location.0 as usize + 1][location.1 as usize + 1] {
                location.0 += 1;
                location.1 += 1;
                continue;
            } else {
                filled[location.0 as usize][location.1 as usize] = true;
                println!("Grain {} stopped at {:?}", grain, location);
                break;
            }
        }
    }
}

fn paint_line(filled: &mut Vec<Vec<bool>>, line: &[(isize, isize)]) {
    for ((x1, y1), (x2, y2)) in line.iter().tuple_windows() {
        let mut x: isize = *x1;
        let mut y: isize = *y1;

        loop {
            filled[x as usize][y as usize] = true;
            if (x, y) == (*x2, *y2) {
                break;
            }

            x += (x2 - x1).signum();
            y += (y2 - y1).signum();
        }
    }
}
