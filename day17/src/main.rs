use std::{
    env,
    io::{self, BufRead},
    str::FromStr,
};

use regex::{Captures, Regex};

fn main() {
    let task_arg = env::args()
        .nth(1)
        .expect("Argument indicating task number was not provided.");

    let (x, y) = parse_data();

    match task_arg.parse::<u8>() {
        Ok(t) if t == 1 => part1(x, y),
        Ok(t) if t == 2 => part2(x, y),
        _ => eprintln!("Invalid task argument"),
    };
}

fn get_number<T: FromStr>(capture: &Captures, offset: usize) -> T {
    capture
        .get(offset)
        .and_then(|m| m.as_str().parse::<T>().ok())
        .unwrap()
}

fn parse_data() -> ((i64, i64), (i64, i64)) {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let pattern = Regex::new(r"(-?\d+)\.\.(-?\d+)").unwrap();
    let mut captures = pattern.captures_iter(&line);

    let x = captures.next().unwrap();
    let x1: i64 = get_number(&x, 1);
    let x2: i64 = get_number(&x, 2);

    let y = captures.next().unwrap();
    let y1: i64 = get_number(&y, 1);
    let y2: i64 = get_number(&y, 2);

    ((x1, x2), (y1, y2))
}

fn part1(x: (i64, i64), y: (i64, i64)) {
    let h_max = (y.0 * (y.0 + 1)) / 2;
    println!("{:?}", h_max);
}

struct State {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl State {
    fn new(vx: i64, vy: i64) -> Self {
        State { x: 0, y: 0, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        match self.vx {
            v if v > 0 => self.vx -= 1,
            v if v < 0 => self.vx += 1,
            _ => {}
        }

        self.vy -= 1;
    }
}

fn simulate(x: (i64, i64), y: (i64, i64), vx: i64, vy: i64) -> bool {
    let mut state = State::new(vx, vy);

    loop {
        if state.x > x.1 {
            return false;
        }

        if state.y < y.0 {
            return false;
        }

        state.step();

        if x.0 <= state.x && state.x <= x.1 {
            if y.0 <= state.y && state.y <= y.1 {
                return true;
            }
        }
    }
}

fn part2(x: (i64, i64), y: (i64, i64)) {
    let vx_min = (2. * x.0 as f64 - 1.).sqrt().floor() as i64;
    let vx_max = x.1;

    let vy_min = y.0;
    let vy_max = -y.0;

    let mut total = 0;
    for vx in vx_min..(vx_max + 1) {
        for vy in vy_min..(vy_max + 1) {
            if simulate(x, y, vx, vy) {
                total += 1;
            }
        }
    }

    println!("{}", total);
}
