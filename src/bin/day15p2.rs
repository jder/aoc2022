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

struct Sensor {
    pos: (isize, isize),
    closest_beacon: (isize, isize),
}

fn main() {
    let line_re = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();
    let sensors = fs::read_to_string("input/day15.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let cap = line_re.captures(line).unwrap();
            Sensor {
                pos: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                closest_beacon: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            }
        })
        .collect_vec();

    let max_coord = 4_000_000;
    'outer: for y in 0..=max_coord {
        if y % 1000 == 0 {
            println!("{y}");
        }
        let mut x = 0;
        loop {
            let beacon = (x, y);
            if let Some(skip) = sensors
                .iter()
                .find_map(|sensor| sensor_excludes(sensor, beacon))
            {
                assert!(skip > 0);
                x += skip;
            } else {
                println!("{x}, {y} = {}", x * max_coord + y);
                break 'outer;
            }
            if x > max_coord {
                break;
            }
        }
    }
}

fn distance((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn sensor_excludes(sensor: &Sensor, beacon: (isize, isize)) -> Option<isize> {
    let delta = distance(sensor.pos, sensor.closest_beacon) - distance(sensor.pos, beacon);

    if delta >= 0 {
        if delta == 0 || beacon == sensor.closest_beacon {
            Some(1)
        } else {
            Some(delta)
        }
    } else {
        None
    }
}
