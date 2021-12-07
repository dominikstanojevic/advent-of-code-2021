use std::{
    env,
    io::{self, BufRead},
};

fn main() {
    let task_arg = env::args()
        .nth(1)
        .expect("Argument indicating task number was not provided.");

    let stdin = io::stdin();
    let line = stdin.lock().lines().nth(0).and_then(|s| s.ok()).unwrap();
    let values: Vec<i64> = line.trim().split(",").map(|s| s.parse().unwrap()).collect();

    match task_arg.parse::<u8>() {
        Ok(t) if t == 1 => part1(values),
        Ok(t) if t == 2 => part2(values),
        _ => eprintln!("Invalid task argument"),
    };
}

fn part2(values: Vec<i64>) {
    let mean: i64 = values.iter().sum::<i64>() / values.len() as i64;

    let cost_mu_down: i64 = values
        .iter()
        .map(|v| calculate_cost((*v - mean).abs()))
        .sum();
    let cost_mu_up: i64 = values
        .iter()
        .map(|v| calculate_cost((*v - mean - 1).abs()))
        .sum();

    println!("{}", cost_mu_down.min(cost_mu_up));
}

fn part1(mut values: Vec<i64>) {
    values.sort();
    let median = match values.len() {
        l if l / 2 == 0 => {
            let p = l / 2;
            (values[p] + values[p + 1]) / 2
        }
        l => {
            let p = l / 2;
            values[p]
        }
    };

    let cost: i64 = values.iter().map(|v| (*v - median).abs()).sum();
    println!("{}", cost);
}

fn calculate_cost(dist: i64) -> i64 {
    return dist * (dist + 1) / 2;
}
