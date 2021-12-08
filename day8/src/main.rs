use std::{
    env,
    io::{self, BufRead},
    ops::Deref,
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

fn part1(lines: Vec<String>) {
    let var = lines
        .iter()
        .flat_map(|s| s.trim().split("|").nth(1).unwrap().trim().split(" "))
        .filter(|s| is_unique(s))
        .count();
    println!("{}", var);
}

fn sort_string(string: &str) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));

    String::from_iter(chars)
}

fn part2(lines: Vec<String>) {}

fn is_unique(string: &str) -> bool {
    match string.len() {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}
