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

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
pub enum Item {
    Number(i64),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.partial_cmp(b),
            (Item::List(a), Item::List(b)) => a.partial_cmp(b),
            (Item::Number(a), Item::List(_)) => {
                Item::List(vec![Item::Number(*a)]).partial_cmp(other)
            }
            (Item::List(_), Item::Number(b)) => {
                self.partial_cmp(&Item::List(vec![Item::Number(*b)]))
            }
        }
    }
}

peg::parser! {
    grammar list_parser() for str {
        rule number() -> Item = n:$(['0'..='9']+) { Item::Number(n.parse().unwrap()) }

        rule item() -> Item = number() / list()

        pub rule list() -> Item = "[" i:(item() ** ",") "]" { Item::List(i) }
    }
}

fn main() {
    let mut sum = 0;
    for (i, (a, b, _)) in fs::read_to_string("input/day13.txt")
        .expect("Failed to read input file")
        .lines()
        .tuples()
        .enumerate()
    {
        println!("{} {}", a, b);
        let a = list_parser::list(a).unwrap();
        let b = list_parser::list(b).unwrap();

        if a < b {
            sum += i + 1;
            println!("a < b");
        } else if a > b {
            println!("a > b");
        } else {
            panic!("a == b");
        }
    }

    println!("Sum: {}", sum);
}
