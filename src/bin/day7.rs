#![allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input/day7.txt").expect("Failed to read input file");

    let mut sizes = HashMap::<_, u32>::new();
    let mut current = Path::new("/").to_path_buf();
    let mut all_dirs = HashSet::new();

    for line in content.lines() {
        if let Some(r) = regex::Regex::new(r"\$ cd (.*)").unwrap().captures(line) {
            if &r[1] == "/" {
                current = Path::new("/").into();
            } else if &r[1] == ".." {
                current.pop();
            } else {
                current.push(&r[1]);
                all_dirs.insert(current.clone());
                println!("cd {}", current.display());
            }
        } else if let Some(r) = regex::Regex::new(r"(\d+) (.*)").unwrap().captures(line) {
            sizes.insert(current.join(&r[2]), r[1].parse().unwrap());
            println!("{}: {}", current.join(&r[2]).display(), &r[1]);
        }
    }

    let result: u32 = all_dirs
        .iter()
        .filter_map(|dir| {
            let size: u32 = sizes
                .iter()
                .filter_map(|(k, v)| if k.starts_with(dir) { Some(v) } else { None })
                .sum();
            if size <= 100000 {
                Some(size)
            } else {
                None
            }
        })
        .sum();

    println!("Result: {}", result);
}
