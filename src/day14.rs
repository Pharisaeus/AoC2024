use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::ops::{Add, Div, Mul};

struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(data: &str) -> Vector {
        let (x, y) = data[2..]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        Vector { x, y }
    }

    fn reduce(&self, max_x: i32, max_y: i32) -> Vector {
        Vector {
            x: (self.x + max_x) % max_x,
            y: (self.y + max_y) % max_y,
        }
    }

    fn distance(&self, rhs: &Vector) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

struct Robot {
    pos: Vector,
    velocity: Vector,
}
impl Robot {
    fn steps(&self, count: i32, max_x: i32, max_y: i32) -> Robot {
        Robot {
            pos: (&self.pos + &(&self.velocity.reduce(max_x, max_y) * count)).reduce(max_x, max_y),
            velocity: Vector {
                x: self.velocity.x,
                y: self.velocity.y,
            },
        }
    }

    fn quadrant(&self, max_x: i32, max_y: i32) -> Option<(i32, i32)> {
        let a = self.find_split(max_x, self.pos.x);
        let b = self.find_split(max_y, self.pos.y);
        a.map(|x| b.map(|y| (x, y))).flatten()
    }

    fn find_split(&self, max: i32, pos: i32) -> Option<i32> {
        let mid = max.div(2) + 1;
        if (pos + 1) % mid == 0 {
            None
        } else {
            Some((pos + 1) / mid)
        }
    }
}

fn display(robots: &Vec<Robot>, max_x: i32, max_y: i32) -> String {
    let taken: HashSet<(i32, i32)> = robots.iter().map(|r| (r.pos.x, r.pos.y)).collect();
    let mut result = String::new();
    for y in 0..max_y {
        for x in 0..max_x {
            if taken.contains(&(x, y)) {
                result.push_str("R");
            } else {
                result.push_str(".");
            }
        }
        result.push('\n');
    }
    result.push('\n');
    result
}

fn part2(robots: &Vec<Robot>) -> i32 {
    let max_x = 101;
    let max_y = 103;
    let mut step = 1;
    loop {
        let current = robots.iter().map(|r| r.steps(step, max_x, max_y)).collect();
        if has_tree(&current) {
            println!("{}", display(&current, max_x, max_y));
            return step;
        }
        step += 1;
    }
}

fn has_tree(robots: &Vec<Robot>) -> bool {
    let mut buddies = 0;
    for r in robots {
        for r2 in robots {
            if r.pos.distance(&r2.pos) == 1 {
                buddies += 1;
                break;
            }
        }
    }
    buddies > robots.len() / 2
}

fn part1(robots: &Vec<Robot>) -> usize {
    let max_x = 101;
    let max_y = 103;
    let quadrant_count = robots
        .iter()
        .map(|r| r.steps(100, max_x, max_y))
        .map(|r| r.quadrant(max_x, max_y))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .counts();
    quadrant_count.values().product()
}
pub(crate) fn solve() {
    let content = fs::read_to_string("14.txt").unwrap();
    let robots = content
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|c| Vector::new(c))
                .collect_tuple()
                .unwrap()
        })
        .map(|(pos, velocity)| Robot { pos, velocity })
        .collect();
    println!("{}", part1(&robots));
    println!("{}", part2(&robots));
}
