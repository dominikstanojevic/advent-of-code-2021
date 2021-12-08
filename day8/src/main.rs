use std::{
    collections::HashSet,
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

fn part2(lines: Vec<String>) {
    let mut count = 0;
    for line in &lines {
        let mut data = line.trim().split("|");

        let mut inputs: Vec<String> = data
            .nth(0)
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| sort_string(s))
            .collect();
        inputs.sort_by(|s1, s2| s1.len().cmp(&s2.len()));

        let mut indices: [u64; 10] = [1, 7, 4, 0, 0, 0, 0, 0, 0, 8];

        for i in 3..6 {
            // 2, 3, 5
            let value = match &inputs[i] {
                s if intersection_len(s, &inputs[1]) == 3 => 3,
                s if intersection_len(s, &inputs[2]) == 2 => 2,
                _ => 5,
            };

            indices[i] = value;
        }

        for i in 6..9 {
            // 0, 6, 9
            let value = match &inputs[i] {
                s if intersection_len(s, &inputs[2]) == 4 => 9,
                s if intersection_len(s, &inputs[0]) == 1 => 6,
                _ => 0,
            };

            indices[i] = value;
        }

        let sum: u64 = data
            .nth(0)
            .unwrap()
            .trim()
            .split(" ")
            .enumerate()
            .map(|(i, s)| {
                indices[inputs.iter().position(|ts| ts == &sort_string(s)).unwrap()]
                    * 10u64.pow(3 - i as u32)
            })
            .sum();

        count += sum;
    }

    println!("{}", count);
}

fn is_unique(string: &str) -> bool {
    match string.len() {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

fn intersection_len(str1: &str, str2: &str) -> usize {
    let str1: HashSet<char> = str1.chars().collect();
    let str2: HashSet<char> = str2.chars().collect();

    str1.intersection(&str2).count()
}
