use itertools::Itertools;
use std::fs;
use std::ops::{Add, Mul};

struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn is_sat_add_mul(&self) -> bool {
        let potential_operators = [u64::add, u64::mul].iter().collect();
        self.is_sat(&potential_operators)
    }

    fn is_sat_add_mul_concat(&self) -> bool {
        let potential_operators = [u64::add, u64::mul, Equation::concat].iter().collect();
        self.is_sat(&potential_operators)
    }

    fn concat(x: u64, y: u64) -> u64 {
        let shift = y.to_string().len() as u32;
        x * 10_u64.pow(shift) + y
    }
    fn is_sat(&self, potential_operators: &Vec<&fn(u64, u64) -> u64>) -> bool {
        let sequence_length = self.numbers.len();
        let operators_variations = (0..sequence_length)
            .map(|_| potential_operators)
            .multi_cartesian_product();
        for operator_sequence in operators_variations {
            let mut res = self.numbers[0];
            for (number, op) in self.numbers[1..].iter().zip(operator_sequence) {
                res = op(res, *number);
                if res > self.result {
                    break;
                }
            }
            if res == self.result {
                return true;
            }
        }
        false
    }
}

fn parse_line(line: &str) -> Equation {
    let (result, numbers) = line.split(": ").collect_tuple().unwrap();
    Equation {
        result: result.parse().unwrap(),
        numbers: numbers.split(" ").map(|x| x.parse().unwrap()).collect(),
    }
}

fn part2(equations: &Vec<Equation>) -> u64 {
    equations
        .iter()
        .filter(|&eq| eq.is_sat_add_mul_concat())
        .map(|eq| eq.result)
        .sum()
}
fn part1(equations: &Vec<Equation>) -> u64 {
    equations
        .iter()
        .filter(|&eq| eq.is_sat_add_mul())
        .map(|eq| eq.result)
        .sum()
}

pub(crate) fn solve() {
    let content = fs::read_to_string("7.txt").unwrap();
    let equations = content.lines().map(parse_line).collect();
    println!("{}", part1(&equations));
    println!("{}", part2(&equations));
}
