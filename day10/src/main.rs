use lazy_static::lazy_static;
use std::{
    collections::{HashMap, VecDeque},
    env,
    io::{self, BufRead},
};

fn main() {
    let task_arg = env::args()
        .nth(1)
        .expect("Argument indicating task number was not provided.");

    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|s| s.unwrap()).collect();

    match task_arg.parse::<u8>() {
        Ok(t) if t == 1 => part1(lines),
        Ok(t) if t == 2 => part2(lines),
        _ => eprintln!("Invalid task argument"),
    };
}

fn error_values(symbol: char) -> u64 {
    match symbol {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn part1(lines: Vec<String>) {
    let mut errs = 0;
    for line in lines {
        let mut stack = VecDeque::new();

        for c in line.chars() {
            if c == '(' || c == '[' || c == '<' || c == '{' {
                stack.push_back(c);
            } else {
                if is_corrupted(c, stack.pop_back().unwrap()) {
                    errs += error_values(c);
                    break;
                }
            }
        }
    }

    println!("{}", errs);
}

fn is_corrupted(symbol: char, stack_top: char) -> bool {
    match symbol {
        ')' => stack_top != '(',
        ']' => stack_top != '[',
        '}' => stack_top != '{',
        '>' => stack_top != '<',
        _ => unreachable!(),
    }
}

fn part2(lines: Vec<String>) {
    let mut total = Vec::new();
    for line in lines {
        let mut stack = VecDeque::new();
        let mut discard = false;

        for c in line.chars() {
            if c == '(' || c == '[' || c == '<' || c == '{' {
                stack.push_back(c);
            } else {
                if is_corrupted(c, stack.pop_back().unwrap()) {
                    discard = true;
                    break;
                }
            }
        }

        if discard || stack.is_empty() {
            continue;
        }

        let mut score: u64 = 0;
        while let Some(c) = stack.pop_back() {
            score = score * 5 + get_score(c);
        }
        total.push(score);
    }

    total.sort();
    println!("{}", total[total.len() / 2]);
}

fn get_score(symbol: char) -> u64 {
    match symbol {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}
