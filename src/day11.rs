use std::collections::{HashMap, VecDeque};
use std::fs;

struct Stones {
    stones: VecDeque<u128>,
}

impl Stones {
    fn new(content: &str) -> Self {
        Self {
            stones: content
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect(),
        }
    }
    fn blink(&self, steps: usize) -> usize {
        let mut known = HashMap::new();
        self.stones
            .iter()
            .map(|s| self.expand_stone(*s, steps, &mut known))
            .sum()
    }

    fn expand_stone(
        &self,
        stone: u128,
        steps_left: usize,
        known: &mut HashMap<(u128, usize), usize>,
    ) -> usize {
        if steps_left == 0 {
            return 1;
        }
        if known.contains_key(&(stone, steps_left)) {
            *known.get(&(stone, steps_left)).unwrap()
        } else {
            let digits = stone.to_string();
            let res = if stone.eq(&0) {
                self.expand_stone(1, steps_left - 1, known)
            } else if digits.len() % 2 == 0 {
                let left = digits[0..digits.len() / 2].parse::<u128>().unwrap();
                let right = digits[digits.len() / 2..].parse::<u128>().unwrap();
                self.expand_stone(left, steps_left - 1, known)
                    + self.expand_stone(right, steps_left - 1, known)
            } else {
                self.expand_stone(stone * 2024, steps_left - 1, known)
            };
            known.insert((stone, steps_left), res);
            res
        }
    }
}

fn part2(stones: &Stones) -> usize {
    stones.blink(75)
}

fn part1(stones: &Stones) -> usize {
    stones.blink(25)
}
pub(crate) fn solve() {
    let content = fs::read_to_string("11.txt").unwrap();
    let stones = Stones::new(&content);
    println!("{}", part1(&stones));
    println!("{}", part2(&stones));
}
