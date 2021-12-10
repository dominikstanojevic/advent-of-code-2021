use std::{
    collections::{HashSet, VecDeque},
    env,
    io::{self, BufRead},
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
    let matrix = get_map(lines);
    let n_rows = matrix.len();
    let n_cols = *&matrix[0].len();

    let mut sum: u64 = 0;
    for i in 1..(n_rows - 1) {
        for j in 1..(n_cols - 1) {
            let val = &matrix[i][j];
            if *val < *&matrix[i - 1][j]
                && *val < *&matrix[i + 1][j]
                && *val < *&matrix[i][j - 1]
                && *val < *&matrix[i][j + 1]
            {
                sum += *val as u64 + 1;
            }
        }
    }

    println!("{}", sum);
}

fn part2(lines: Vec<String>) {
    let matrix = get_map(lines);
    let n_rows = matrix.len();
    let n_cols = *&matrix[0].len();

    let mut basins: Vec<u64> = Vec::new();
    for i in 1..(n_rows - 1) {
        for j in 1..(n_cols - 1) {
            let val = &matrix[i][j];
            if val < &matrix[i - 1][j]
                && val < &matrix[i + 1][j]
                && val < &matrix[i][j - 1]
                && val < &matrix[i][j + 1]
            {
                let basin_len = find_basin_len(&matrix, i, j);
                basins.push(basin_len);
            }
        }
    }

    basins.sort();
    let result: u64 = basins.iter().rev().take(3).product();

    println!("{}", result);
}

fn get_map(lines: Vec<String>) -> Vec<Vec<u8>> {
    let n_cols = &lines[0].len() + 2;
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    matrix.push(vec![u8::MAX; n_cols]);

    for line in lines {
        let mut nums: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        nums.insert(0, u8::MAX);
        nums.push(u8::MAX);

        matrix.push(nums);
    }
    matrix.push(vec![u8::MAX; n_cols]);

    matrix
}

fn find_basin_len(matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> u64 {
    let mut unexplored_positions = VecDeque::from([(i, j)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut length = 0;

    while let Some((i, j)) = unexplored_positions.pop_front() {
        let val = matrix[i][j];

        // We stop here
        if val >= 9 {
            continue;
        }

        if visited.insert((i, j)) {
            length += 1;
        } else {
            continue;
        }

        if val < matrix[i - 1][j] {
            unexplored_positions.push_back((i - 1, j))
        }
        if val < matrix[i + 1][j] {
            unexplored_positions.push_back((i + 1, j))
        }
        if val < matrix[i][j - 1] {
            unexplored_positions.push_back((i, j - 1))
        }
        if val < matrix[i][j + 1] {
            unexplored_positions.push_back((i, j + 1))
        }
    }

    length
}
