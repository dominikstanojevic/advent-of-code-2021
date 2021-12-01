use std::{
    collections::VecDeque,
    env,
    io::{self, BufRead},
};

fn part1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut previous = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
    let mut larger = 0u64;
    while let Some(line) = lines.next() {
        let current = line.unwrap().parse::<i64>().unwrap();
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

    let mut deque = VecDeque::new();
    let mut larger = 0u64;
    while let Some(line) = lines.next() {
        let value = line.unwrap().parse::<i64>().unwrap();

        if deque.len() == 3 {
            let previous: i64 = deque.iter().sum();
            deque.pop_front();

            deque.push_back(value);
            let current: i64 = deque.iter().sum();

            if current > previous {
                larger += 1;
            }
        } else {
            deque.push_back(value);
        }
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
