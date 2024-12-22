use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn next_number(secret: i64) -> i64 {
    let mut result = (secret ^ (secret << 6)) % 16777216;
    result = (result ^ (result >> 5)) % 16777216;
    result = (result ^ (result << 11)) % 16777216;
    result
}

fn skip(secret: &i64, steps: usize) -> i64 {
    let mut res = *secret;
    for _ in 0..steps {
        res = next_number(res);
    }
    res
}

fn price_changes(s: &i64) -> Vec<(i64, i64)> {
    let mut res = vec![];
    let mut current = *s;
    let mut last_price = current % 10;
    for _ in 0..2000 {
        current = next_number(current);
        let price = current % 10;
        res.push((price - last_price, price));
        last_price = price;
    }
    res
}

fn compute_sequences(s: &i64) -> HashMap<Vec<i64>, i64> {
    let mut res = HashMap::new();
    let changes = price_changes(s);
    for (a, b, c, d) in changes.iter().tuple_windows() {
        let seq = vec![a.0, b.0, c.0, d.0];
        let price = d.1;
        if !res.contains_key(&seq) {
            res.insert(seq, price);
        }
    }
    res
}

fn part2(secrets: &Vec<i64>) -> i64 {
    let sequences: Vec<HashMap<Vec<i64>, i64>> =
        secrets.iter().map(|s| compute_sequences(s)).collect();
    let potential_sequences: Vec<Vec<i64>> = sequences
        .iter()
        .map(|s| s.keys())
        .flatten()
        .cloned()
        .unique()
        .collect();
    let mut best = 0;
    for s in potential_sequences {
        let new_score = sequences.iter().map(|x| x.get(&s).unwrap_or(&0)).sum();
        if new_score > best {
            best = new_score;
        }
    }
    best
}

fn part1(secrets: &Vec<i64>) -> i64 {
    secrets.iter().map(|s| skip(s, 2000)).sum()
}
pub(crate) fn solve() {
    let content = fs::read_to_string("22.txt").unwrap();
    let secrets = content.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    println!("{}", part1(&secrets));
    println!("{}", part2(&secrets));
}
