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
    let content = fs::read_to_string("input/day8.txt").expect("Failed to read input file");

    let mut grid = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap(), false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let num_cols = grid[0].len();
    println!("Num cols: {}", num_cols);

    for row in 0..grid.len() {
        mark_visible((0..num_cols).map(|col| (row, col)), &mut grid);
        mark_visible((0..num_cols).rev().map(|col| (row, col)), &mut grid);
    }

    for col in 0..num_cols {
        mark_visible((0..grid.len()).map(|row| (row, col)), &mut grid);
        mark_visible((0..grid.len()).rev().map(|row| (row, col)), &mut grid);
    }

    println!(
        "Result: {}",
        grid.iter()
            .map(|row| row
                .iter()
                .map(|(_, visible)| if *visible { 1 } else { 0 })
                .sum::<usize>())
            .sum::<usize>()
    );
}

fn mark_visible(iter: impl Iterator<Item = (usize, usize)>, grid: &mut Vec<Vec<(u32, bool)>>) {
    let mut max_value: i32 = -1;
    for (row, col) in iter {
        let (value, visible) = &mut grid[row][col];
        if *value as i32 > max_value {
            max_value = *value as i32;
            *visible = true;
        }
    }
}
