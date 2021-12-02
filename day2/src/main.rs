use std::{
    env,
    io::{self, BufRead},
    str::FromStr,
};

enum Direction {
    FORWARD,
    UP,
    DOWN,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "forward" => Ok(Self::FORWARD),
            "up" => Ok(Self::UP),
            "down" => Ok(Self::DOWN),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn step_part1(&self, value: i64, horizontal: i64, depth: i64) -> (i64, i64) {
        match self {
            Self::FORWARD => return (horizontal + value, depth),
            Self::UP => return (horizontal, depth - value),
            Self::DOWN => return (horizontal, depth + value),
        }
    }

    fn step_part2(&self, value: i64, horizontal: i64, depth: i64, aim: i64) -> (i64, i64, i64) {
        match self {
            Self::FORWARD => return (horizontal + value, depth + value * aim, aim),
            Self::UP => return (horizontal, depth, aim - value),
            Self::DOWN => return (horizontal, depth, aim + value),
        }
    }
}

fn parse_line(line: &str) -> (Direction, i64) {
    let mut line = line.trim().split(" ");

    let direction = Direction::from_str(line.next().unwrap()).unwrap();
    let value = line.next().unwrap().parse::<i64>().unwrap();

    (direction, value)
}

fn part1() {
    let stdin = io::stdin();
    let result = stdin
        .lock()
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .fold((0, 0), |(h, d), l| l.0.step_part1(l.1, h, d));

    println!("{}", result.0 * result.1);
}

fn part2() {
    let stdin = io::stdin();
    let result = stdin
        .lock()
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .fold((0, 0, 0), |(h, d, a), l| l.0.step_part2(l.1, h, d, a));

    println!("{}", result.0 * result.1);
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
