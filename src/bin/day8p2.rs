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

    let grid = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let num_cols = grid[0].len();

    let result = (0..grid.len())
        .flat_map(|row| (0..num_cols).map(move |col| (row, col)))
        .map(|(row, col)| {
            let result = scenic(row, col, &grid);
            println!("({}, {}): {}", row, col, result);
            result
        })
        .max();

    println!("Result: {}", result.unwrap());
}

fn scenic(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let num_cols = grid[0].len();

    return distance(
        grid[row][col],
        (row + 1..grid.len()).map(|row| (row, col)),
        grid,
    ) * distance(grid[row][col], (0..row).rev().map(|row| (row, col)), grid)
        * distance(
            grid[row][col],
            (col + 1..num_cols).map(|col| (row, col)),
            grid,
        )
        * distance(grid[row][col], (0..col).rev().map(|col| (row, col)), grid);
}

fn distance(start: u32, iter: impl Iterator<Item = (usize, usize)>, grid: &Vec<Vec<u32>>) -> u32 {
    let mut distance = 0;
    for (row, col) in iter {
        let value = grid[row][col] as i32;
        distance += 1;

        if value >= start as i32 {
            break;
        }
    }
    return distance;
}
