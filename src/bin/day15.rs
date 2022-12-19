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

    let max_x = sensors
        .iter()
        .map(|sensor| distance(sensor.pos, sensor.closest_beacon) + sensor.pos.0)
        .max()
        .unwrap();

    let min_x = sensors
        .iter()
        .map(|sensor| sensor.pos.0 - distance(sensor.pos, sensor.closest_beacon))
        .min()
        .unwrap();

    let mut excluded = 0;
    let y = 2_000_000;
    for x in min_x..=max_x {
        let beacon = (x, y);
        if sensors.iter().any(|sensor| sensor_excludes(sensor, beacon)) {
            excluded += 1;
        }
    }
    println!("{excluded}");
}

fn distance((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn sensor_excludes(sensor: &Sensor, beacon: (isize, isize)) -> bool {
    distance(sensor.pos, beacon) <= distance(sensor.pos, sensor.closest_beacon)
        && beacon != sensor.closest_beacon
}
