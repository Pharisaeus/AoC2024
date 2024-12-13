use regex::Regex;
use std::fs;

struct Machine {
    a: (i128, i128),
    b: (i128, i128),
    prize: (i128, i128),
}

impl Machine {
    fn new(block: &str) -> Self {
        let re = Regex::new(r"X.?(\d+).*Y.?(\d+)").unwrap();
        let mut res = vec![];
        for capture in re.captures_iter(block) {
            let x = capture.get(1).unwrap().as_str().parse::<i128>().unwrap();
            let y = capture.get(2).unwrap().as_str().parse::<i128>().unwrap();
            res.push((x, y));
        }
        Self {
            a: res[0],
            b: res[1],
            prize: res[2],
        }
    }

    fn small_prize(&self) -> (i128, i128) {
        self.prize
    }

    fn big_prize(&self) -> (i128, i128) {
        (self.prize.0 + 10000000000000, self.prize.1 + 10000000000000)
    }

    fn solve_small(&self) -> i128 {
        self.solve(self.small_prize())
    }
    fn solve_big(&self) -> i128 {
        self.solve(self.big_prize())
    }

    fn solve(&self, prize: (i128, i128)) -> i128 {
        let ax = self.a.0;
        let ay = self.a.1;
        let bx = self.b.0;
        let by = self.b.1;
        let (px, py) = prize;
        let k = (bx * py - by * px) / (ay * bx - ax * by);
        let m = (ay * px - ax * py) / (ay * bx - ax * by);
        if self.check(k, m, prize) {
            self.cost(k, m)
        } else {
            0
        }
    }

    fn check(&self, k: i128, m: i128, prize: (i128, i128)) -> bool {
        (prize.0 == self.a.0 * k + self.b.0 * m) && (prize.1 == self.a.1 * k + self.b.1 * m)
    }

    fn cost(&self, k: i128, m: i128) -> i128 {
        k * 3 + m
    }
}
fn part2(machines: &Vec<Machine>) -> i128 {
    machines.iter().map(|m| m.solve_big()).sum()
}
fn part1(machines: &Vec<Machine>) -> i128 {
    machines.iter().map(|m| m.solve_small()).sum()
}
pub(crate) fn solve() {
    let content = fs::read_to_string("13.txt").unwrap();
    let machines = content
        .split("\n\n")
        .map(|block| Machine::new(block))
        .collect();
    println!("{}", part1(&machines));
    println!("{}", part2(&machines));
}
