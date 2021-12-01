use std::{
    collections::VecDeque,
    env,
    io::{self, BufRead},
};

fn part1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut previous = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    let mut larger = 0u32;
    while let Some(line) = lines.next() {
        let current = line.unwrap().parse::<u32>().unwrap();
        if current > previous {
            larger += 1;
        }

        previous = current;
    }

    println!("In total there were {} larger measurements.", larger);
}

fn part2() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut deque: VecDeque<u32> = (&mut lines)
        .take(3)
        .map(|v| v.unwrap().parse::<u32>().unwrap())
        .collect();
    let mut previous: u32 = deque.iter().sum();

    let mut larger = 0u32;
    while let Some(line) = lines.next() {
        let removed = deque.pop_front().unwrap();

        let value = line.unwrap().parse::<u32>().unwrap();
        deque.push_back(value);
        let current: u32 = previous - removed + value;

        if current > previous {
            larger += 1;
        }

        previous = current;
    }

    println!("In total there were {} larger measurements.", larger);
}

fn main() {
    let task_arg = env::args()
        .nth(1)
        .expect("Argument indicating task number was not provided.");

    match task_arg.parse::<u8>() {
        Ok(t) if t == 1 => part1(),
        Ok(t) if t == 2 => part2(),
        _ => eprintln!("Invalid task argument"),
    };
}
