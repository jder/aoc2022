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
    let divider1 = list_parser::list("[[2]]").unwrap();
    let divider2 = list_parser::list("[[6]]").unwrap();

    let mut full_list = fs::read_to_string("input/day13.txt")
        .unwrap()
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| list_parser::list(line).unwrap())
        .collect_vec();

    full_list.push(divider1.clone());
    full_list.push(divider2.clone());

    full_list.sort();

    println!("{:?}", full_list);

    println!(
        "{}",
        (full_list.iter().position(|item| item == &divider1).unwrap() + 1)
            * (1 + full_list.iter().position(|item| item == &divider2).unwrap())
    );
}
