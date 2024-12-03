use itertools::Itertools;
use regex::{Captures, Regex};
use std::fs;

fn part2(contents: &String) -> u32 {
    let re = Regex::new(r"mul\((\d+,\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for capture in re.captures_iter(contents) {
        let instruction = capture.get(0).unwrap().as_str();
        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            &_ => {
                if enabled {
                    sum += compute(&capture)
                }
            }
        };
    }
    sum
}
fn part1(contents: &String) -> u32 {
    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    re.captures_iter(contents.as_str())
        .map(|capture| compute(&capture))
        .sum()
}

fn compute(c: &Captures) -> u32 {
    c.get(1)
        .unwrap()
        .as_str()
        .split(",")
        .collect_vec()
        .iter()
        .map(|y| y.parse::<u32>().unwrap())
        .product()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("3.txt").unwrap();
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}
