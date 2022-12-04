#[allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use std::fs;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "X" | "A" => Some(Self::Rock),
            "Y" | "B" => Some(Self::Paper),
            "Z" | "C" => Some(Self::Scissors),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn outcome(&self, other: &Self) -> u32 {
        match (self, other) {
            (Choice::Rock, Choice::Paper) => 0,
            (Choice::Rock, Choice::Scissors) => 6,
            (Choice::Paper, Choice::Rock) => 6,
            (Choice::Paper, Choice::Scissors) => 0,
            (Choice::Scissors, Choice::Rock) => 0,
            (Choice::Scissors, Choice::Paper) => 6,
            _ => 3,
        }
    }
}

fn main() {
    let result: u32 = fs::read_to_string("input/day2.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let (theirs, mine) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let theirs = Choice::from_str(theirs).unwrap();
            let mine = Choice::from_str(mine).unwrap();
            mine.outcome(&theirs) + mine.score()
        })
        .sum();
    println!("Result: {:?}", result);
}
