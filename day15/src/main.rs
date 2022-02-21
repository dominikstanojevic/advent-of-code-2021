use std::{
    cmp::Reverse,
    collections::BinaryHeap,
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

fn parse_data() -> Vec<Vec<u8>> {
    let stdin = io::stdin();
    let data = stdin
        .lock()
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    data
}

fn part1(data: Vec<Vec<u8>>) {
    let n_rows = 5 * data.len();
    let n_cols = 5 * data[0].len();

    let mut dist = vec![vec![u64::MAX; n_cols]; n_rows];
    dist[0][0] = 0;

    let mut queue = BinaryHeap::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            queue.push(Reverse((dist[i][j], i, j)));
        }
    }

    while !queue.is_empty() {
        let (_, i, j) = queue.pop().unwrap().0;

        for pos in get_neighbors(i, j, n_rows, n_cols) {
            let (ii, jj) = pos;

            let d = dist[i][j] + get_cost(&data, ii, jj);
            if d < dist[ii][jj] {
                dist[ii][jj] = d;
                queue.push(Reverse((d, ii, jj)));
            }
        }
    }

    println!("{}", dist[n_rows - 1][n_cols - 1]);
}

fn get_cost(data: &Vec<Vec<u8>>, i: usize, j: usize) -> u64 {
    let n_rows = data.len();
    let q_i = i / n_rows;
    let r_i = i - q_i * n_rows;

    let n_cols = data[0].len();
    let q_j = j / n_cols;
    let r_j = j - q_j * n_cols;

    let cost = data[r_i][r_j] as u64 + q_i as u64 + q_j as u64;
    match cost {
        c if c >= 10 => c - 9,
        c => c,
    }
}

fn get_neighbors(i: usize, j: usize, n_rows: usize, n_cols: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if i < n_rows - 1 {
        neighbors.push((i + 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if j < n_cols - 1 {
        neighbors.push((i, j + 1));
    }

    neighbors
}

fn part2(data: Vec<Vec<u8>>) {}
