use std::{
    collections::HashSet,
    env,
    hash::Hash,
    io::{self, BufRead},
    iter::FromIterator,
};

fn parse_line(string: &str) -> u32 {
    u32::from_str_radix(string, 2).unwrap()
}

fn part1() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|s| s.unwrap()).collect();

    let n_digits = values[0].len();
    let mut counter = vec![0u32; n_digits];

    for value in values.iter() {
        for (i, c) in value.chars().enumerate() {
            if c == '1' {
                counter[i] += 1;
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for c in counter {
        if (2 * c as usize) >= values.len() {
            gamma.push('1');
            epsilon.push('0')
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", gamma * epsilon);
}

fn part2() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|s| s.unwrap()).collect();

    let n_digits = values[0].len();

    let mut oxygen = HashSet::from_iter(values.iter().cloned());
    for i in 0..n_digits {
        let mut counter = 0u32;
        for value in oxygen.iter() {
            if value.chars().nth(i).unwrap() == '1' {
                counter += 1;
            }
        }

        let most_common;
        if (counter as usize * 2) >= oxygen.len() {
            most_common = '1';
        } else {
            most_common = '0';
        }

        let mut new = HashSet::new();
        for v in oxygen {
            if v.chars().nth(i).unwrap() == most_common {
                new.insert(v);
            }
        }

        oxygen = new;
    }
    let oxygen = oxygen.into_iter().next().unwrap();

    let mut co2 = HashSet::from_iter(values.iter().cloned());
    for i in 0..n_digits {
        let mut counter = 0u32;
        for value in co2.iter() {
            if value.chars().nth(i).unwrap() == '1' {
                counter += 1;
            }
        }

        let least_common;
        if (counter as usize * 2) >= co2.len() {
            least_common = '0';
        } else {
            least_common = '1';
        }

        let mut new = HashSet::new();
        for v in co2 {
            if v.chars().nth(i).unwrap() == least_common {
                new.insert(v);
            }
        }

        co2 = new;
        if co2.len() == 1 {
            break;
        }
    }

    let co2 = co2.into_iter().next().unwrap();

    let oxygen = u32::from_str_radix(&oxygen, 2).unwrap();
    let co2 = u32::from_str_radix(&co2, 2).unwrap();

    println!("{}", oxygen * co2);
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
