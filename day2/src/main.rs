use std::env;

fn part1() {}

fn part2() {}

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
