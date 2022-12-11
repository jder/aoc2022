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

#[derive(Debug)]
pub enum Value {
    Old,
    Number(i64),
}

#[derive(Debug)]
pub enum Operation {
    Add(Value, Value),
    Mul(Value, Value),
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    throw_target: (usize, usize),
}

peg::parser! {
    grammar monkey_parser() for str {
        rule number() -> i64 = n:$(['0'..='9']+) {? n.parse().or(Err("Invalid number")) }

        rule items() -> Vec<i64> = "Starting items: " l:(number() ** ", ") { l }

        rule value() -> Value = "old" { Value::Old }
            / n:number() { Value::Number(n) }

        rule operation() -> Operation = "Operation: new = " v1:value() " " op:$(['+'|'*']) " " v2:value() {
            if op == "+" {
                Operation::Add(v1, v2)
            } else {
                Operation::Mul(v1, v2)
            }
        }

        rule test() -> i64 = "Test: divisible by " n:number() { n }

        rule throw_target() -> (usize, usize) =
            "If true: throw to monkey " t:number() _
            "If false: throw to monkey " f:number() {
            (t as usize, f as usize)
        }

        rule _() = quiet!{[' ' | '\n' | '\t']*}

        rule monkey() -> Monkey = "Monkey " n:number() ":" _
            items:items() _
            operation:operation() _
            test:test() _
            throw_target:throw_target() _ {
            Monkey {
                items,
                operation,
                test,
                throw_target,
            }
        }

        pub rule monkeys() -> Vec<Monkey> = monkey() ** _
    }
}
fn main() {
    let contents = fs::read_to_string("input/day11.txt").expect("Failed to read input file");
    let mut monkeys = monkey_parser::monkeys(&contents).expect("Failed to parse input file");

    let modulus = monkeys
        .iter()
        .map(|m| m.test)
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap();

    let mut inspections = vec![0; monkeys.len()];

    for round in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[monkey_index].items);
            // inspect the monkey's items
            for mut item in items {
                inspections[monkey_index] += 1;
                apply_operation(&monkeys[monkey_index].operation, &mut item);
                item %= modulus;
                let throw_target = if item % monkeys[monkey_index].test == 0 {
                    // throw to the other monkey
                    monkeys[monkey_index].throw_target.0
                } else {
                    // throw to the other monkey
                    monkeys[monkey_index].throw_target.1
                };

                monkeys[throw_target].items.push(item);
            }
        }
        for monkey_index in 0..monkeys.len() {
            println!(
                "After round {}, monkey {} has {}",
                round,
                monkey_index,
                monkeys[monkey_index].items.iter().join(", ")
            );
        }
    }

    let result: i64 = inspections.iter().sorted().rev().take(2).product();
    println!("Result: {}", result);
}

fn eval_value(value: &Value, item: i64) -> i64 {
    match value {
        Value::Old => item,
        Value::Number(n) => *n,
    }
}

fn apply_operation(operation: &Operation, item: &mut i64) {
    match operation {
        Operation::Add(v1, v2) => {
            *item = eval_value(v1, *item) + eval_value(v2, *item);
        }
        Operation::Mul(v1, v2) => {
            *item = eval_value(v1, *item) * eval_value(v2, *item);
        }
    }
}
