use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

struct Patterns {
    available: HashSet<String>,
    max_len: usize,
    constructable: HashMap<String, usize>,
}

impl Patterns {
    fn new(patterns: &str) -> Patterns {
        let available: HashSet<String> = patterns.split(", ").map(|x| x.to_string()).collect();
        Patterns {
            max_len: available.iter().map(|x| x.len()).max().unwrap(),
            available,
            constructable: HashMap::new(),
        }
    }

    fn can_construct(&mut self, input: &str) -> usize {
        if self.constructable.contains_key(input) {
            return self.constructable[input];
        }
        if input.len() == 0 {
            return 1;
        }
        let mut how_many = 0;
        for prefix_len in (1..=self.max_len.min(input.len())).rev() {
            let prefix = &input[0..prefix_len];
            let suffix = &input[prefix_len..];
            if self.available.contains(prefix) {
                how_many += self.can_construct(suffix);
            }
        }
        self.constructable.insert(input.to_string(), how_many);
        how_many
    }
}

fn part2(patterns: &mut Patterns, towels: &Vec<String>) -> usize {
    towels
        .iter()
        .map(|towel| patterns.can_construct(towel))
        .sum()
}
fn part1(patterns: &mut Patterns, towels: &Vec<String>) -> usize {
    towels
        .iter()
        .filter(|towel| patterns.can_construct(towel) > 0)
        .count()
}
pub(crate) fn solve() {
    let content = fs::read_to_string("19.txt").unwrap();
    let (patterns, towels) = content.split("\n\n").collect_tuple().unwrap();
    let mut available_patterns = Patterns::new(patterns);
    let needed_towels = towels.split("\n").map(|x| x.to_string()).collect();
    println!("{}", part1(&mut available_patterns, &needed_towels));
    println!("{}", part2(&mut available_patterns, &needed_towels));
}
