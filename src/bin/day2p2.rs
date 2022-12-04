#[allow(unused_imports)]
use aoc2022::*;
use itertools::Itertools;
use std::fs;

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_str(s: &str) -> Self {
        match s {
            "Z" => Outcome::Win,
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            _ => panic!("Unknown outcome: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "A" => Some(Self::Rock),
            "B" => Some(Self::Paper),
            "C" => Some(Self::Scissors),
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

    fn for_outcome(&self, outcome: &Outcome) -> Choice {
        match outcome {
            Outcome::Win => match self {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            Outcome::Lose => match self {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Outcome::Draw => self.clone(),
        }
    }
}

fn main() {
    let result: u32 = fs::read_to_string("input/day2.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let (theirs, desired) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let theirs = Choice::from_str(theirs).unwrap();
            let desired = Outcome::from_str(desired);
            let mine = theirs.for_outcome(&desired);
            mine.outcome(&theirs) + mine.score()
        })
        .sum();
    println!("Result: {:?}", result);
}
