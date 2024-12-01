use std::fs;
use itertools::Itertools;

fn part2(first:&Vec<u32>, second:&Vec<u32>) ->u32 {
    let counts = second.iter().counts_by(|x|x);
    first.iter()
        .map(|x| x*(*counts.get(x).unwrap_or(&0) as u32))
        .sum()
}

fn part1(first:&Vec<u32>, second:&Vec<u32>) ->u32 {
    first.iter()
        .sorted()
        .zip(second.iter().sorted())
        .map(|(&x, &y)| x.abs_diff(y))
        .sum()
}

fn parse_line(line: &str) -> (u32, u32) {
    line.split("   ")
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("1.txt").unwrap();
    let numbers:Vec<(u32,u32)> = contents.lines()
        .map(|line|parse_line(line))
        .collect();
    let first = numbers.iter()
        .map(|entry|entry.0)
        .collect();
    let second = numbers.iter()
        .map(|entry|entry.1)
        .collect();
    println!("{}", part1(&first,&second));
    println!("{}", part2(&first,&second));
}
