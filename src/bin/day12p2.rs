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
    let map = fs::read_to_string("input/day12.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let num_cols = map[0].len();

    let find = |c| {
        map.iter().enumerate().find_map(|(row_i, row)| {
            row.iter().enumerate().find_map(|(col_i, col)| {
                if *col == c {
                    Some((row_i, col_i))
                } else {
                    None
                }
            })
        })
    };

    let (start_row, start_col) = find('E').unwrap();

    let height = |c: char| match c {
        'S' => 0,
        'E' => 25,
        _ => c as i32 - 'a' as i32,
    };

    let adjacent = |pos: (usize, usize)| {
        let (row, col) = pos;
        let mut result = vec![];
        if row > 0 {
            result.push((row - 1, col));
        }
        if row < map.len() - 1 {
            result.push((row + 1, col));
        }
        if col > 0 {
            result.push((row, col - 1));
        }
        if col < num_cols - 1 {
            result.push((row, col + 1));
        }
        let map = &map;
        result
            .into_iter()
            .filter(move |(row, col)| height(map[pos.0][pos.1]) - height(map[*row][*col]) <= 1)
    };

    let mut distance = vec![vec![std::i32::MAX; num_cols]; map.len()];
    distance[start_row][start_col] = 0;

    let mut queue = vec![(start_row, start_col)];
    while let Some(pos) = queue.pop() {
        let (row, col) = pos;

        let dist = distance[row][col];
        for (row, col) in adjacent(pos) {
            if distance[row][col] > dist + 1 {
                distance[row][col] = dist + 1;
                queue.push((row, col));
            }
        }
    }

    let mut best = std::i32::MAX;
    for row in 0..map.len() {
        for col in 0..num_cols {
            if map[row][col] == 'a' {
                best = min(best, distance[row][col]);
            }
        }
    }

    println!("{}", best);
}
