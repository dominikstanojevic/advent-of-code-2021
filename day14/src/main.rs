use std::{
    collections::HashMap,
    env,
    io::{self, BufRead},
};

fn main() {
    let task_arg = env::args()
        .nth(1)
        .expect("Argument indicating task number was not provided.");

    let (template, rules) = parse_data();

    match task_arg.parse::<u8>() {
        Ok(t) if t == 1 => task(template, rules, 10),
        Ok(t) if t == 2 => task(template, rules, 40),
        _ => eprintln!("Invalid task argument"),
    };
}

fn parse_data() -> (Vec<char>, HashMap<(char, char), char>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|s| s.unwrap());

    let template = lines.next().unwrap().chars().collect();
    lines.next(); // Skip space

    let rules: HashMap<(char, char), char> = lines
        .map(|s| {
            let mut d = s.trim().split("->");
            let mut left = d.next().unwrap().trim().chars();
            let left: (char, char) = (left.next().unwrap(), left.next().unwrap());

            let right = d.next().and_then(|s| s.trim().chars().next()).unwrap();

            (left, right)
        })
        .collect();

    (template, rules)
}

fn task(template: Vec<char>, rules: HashMap<(char, char), char>, steps: u8) {
    let mut bigrams: HashMap<(char, char), u64> =
        template.windows(2).fold(HashMap::new(), |mut b, w| {
            *b.entry((w[0], w[1])).or_insert(0u64) += 1;
            b
        });

    for _ in 0..steps {
        let mut new_bigrams = HashMap::new();
        for (bigram, count) in bigrams {
            let middle = *rules.get(&bigram).unwrap();
            let left = (bigram.0, middle);
            let right = (middle, bigram.1);

            *new_bigrams.entry(left).or_insert(0u64) += count;
            *new_bigrams.entry(right).or_insert(0u64) += count;
        }

        bigrams = new_bigrams;
    }

    let mut counts =
        bigrams
            .into_iter()
            .map(|(b, c)| (b.0, c))
            .fold(HashMap::new(), |mut count, (l, c)| {
                *count.entry(l).or_insert(0u64) += c;
                count
            });
    *counts.get_mut(&template[template.len() - 1]).unwrap() += 1;

    let (min, max) = counts
        .into_iter()
        .fold((0, 0), |(min, max), (_, c)| (min.min(c), max.max(c)));
    println!("{}", max - min);
}
