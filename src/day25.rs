use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum CellType {
    EMPTY,
    TAKEN,
}

impl CellType {
    fn new(c: &char) -> Self {
        match c {
            '#' => CellType::TAKEN,
            '.' => CellType::EMPTY,
            &_ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    LOCK,
    KEY,
}
struct LockKey {
    cells: HashMap<(i64, i64), CellType>,
    device_type: Type,
}

impl LockKey {
    fn new(block: &str) -> Self {
        let device_type = if block.starts_with("#") {
            Type::LOCK
        } else {
            Type::KEY
        };

        Self {
            cells: block
                .lines()
                .enumerate()
                .map(|(row_index, row)| {
                    row.chars().enumerate().map(move |(col_index, c)| {
                        ((col_index as i64, row_index as i64), CellType::new(&c))
                    })
                })
                .flatten()
                .collect(),
            device_type,
        }
    }

    fn matches(&self, another: &LockKey) -> bool {
        if self.device_type != another.device_type {
            for pos in self.cells.keys() {
                match (self.cells.get(pos), another.cells.get(pos)) {
                    (Some(CellType::TAKEN), Some(CellType::TAKEN)) => return false,
                    _ => {}
                }
            }
            return true;
        }
        false
    }
}

fn part1(data: &Vec<LockKey>) -> i64 {
    let (locks, keys): (Vec<_>, Vec<_>) = data.iter().partition(|&x| x.device_type == Type::LOCK);
    let mut count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.matches(&lock) {
                count += 1;
            }
        }
    }
    count
}
pub(crate) fn solve() {
    let content = fs::read_to_string("25.txt").unwrap();
    let blocks = content.split("\n\n");
    let data = blocks.map(|b| LockKey::new(b)).collect();
    println!("{}", part1(&data));
}
