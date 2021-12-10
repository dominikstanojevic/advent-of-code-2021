use std::{
    collections::VecDeque,
    env,
    io::{self, BufRead},
};

fn main() {
    let task_arg = env::args()
        .nth(1)
        .expect("Argument indicating task number was not provided.");

    let stdin = io::stdin();
    let values = stdin
        .lock()
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    match task_arg.parse::<u8>() {
        Ok(t) if t == 1 => task(values, 80),
        Ok(t) if t == 2 => task(values, 256),
        _ => eprintln!("Invalid task argument"),
    };
}

fn task(values: Vec<u8>, days: usize) {
    let mut counter = VecDeque::from([0u64; 9]);
    values.into_iter().for_each(|v| counter[v as usize] += 1);

    for _ in 0..days {
        let zero_days = counter.pop_front().unwrap();
        counter[6] += zero_days;
        counter.push_back(zero_days);
    }

    let total: u64 = counter.iter().sum();
    println!("{}", total);
}
