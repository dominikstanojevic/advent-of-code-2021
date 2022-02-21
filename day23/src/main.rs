use std::{
    env,
    io::{self, BufRead},
};

fn main() {
    let task_arg = env::args()
        .nth(1)
        .expect("Argument indicating task number was not provided.");

    let data = parse_data();

    match task_arg.parse::<u8>() {
        Ok(t) if t == 1 => part1(data),
        Ok(t) if t == 2 => part2(data),
        _ => eprintln!("Invalid task argument"),
    };
}

#[derive(Clone, Copy)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Amphipod::Amber),
            'B' => Some(Amphipod::Bronze),
            'C' => Some(Amphipod::Copper),
            'D' => Some(Amphipod::Desert),
            _ => unreachable!(),
        }
    }

    fn cost(&self) -> u64 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 100,
        }
    }
}

fn parse_data() -> [[Option<Amphipod>; 2]; 4] {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first = lines.nth(2).unwrap().unwrap().trim().replace("\"", "");
    let second = lines.next().unwrap().unwrap().trim().replace("\"", "");

    let data = [
        [
            Amphipod::from_char(first.chars().nth(0).unwrap()),
            Amphipod::from_char(second.chars().nth(0).unwrap()),
        ],
        [
            Amphipod::from_char(first.chars().nth(1).unwrap()),
            Amphipod::from_char(second.chars().nth(1).unwrap()),
        ],
        [
            Amphipod::from_char(first.chars().nth(2).unwrap()),
            Amphipod::from_char(second.chars().nth(2).unwrap()),
        ],
        [
            Amphipod::from_char(first.chars().nth(3).unwrap()),
            Amphipod::from_char(second.chars().nth(3).unwrap()),
        ],
    ];

    data
}

fn part1(data: [[Option<Amphipod>; 2]; 4]) {
    let hall = [None; 11];
}

fn part2(data: Vec<Vec<Option<Amphipod>>>) {}
