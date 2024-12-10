use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

struct TopoMap {
    cells: HashMap<(i32, i32), u32>,
}

impl TopoMap {
    fn new(content: &str) -> Self {
        let cells = content
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars().enumerate().map(move |(col, ch)| {
                    ((row as i32, col as i32), ch.to_digit(10).unwrap_or(100))
                })
            })
            .flatten()
            .collect();
        Self { cells }
    }

    fn trailheads_score(&self) -> u64 {
        self.trailheads()
            .iter()
            .map(|pos| self.trailhead_score(pos))
            .sum()
    }

    fn trailhead_score(&self, pos: &(i32, i32)) -> u64 {
        self.distinct_9_reachable_from(pos).iter().count() as u64
    }

    fn distinct_9_reachable_from(&self, from: &(i32, i32)) -> Vec<(i32, i32)> {
        if *self.cells.get(from).unwrap() == 9 {
            vec![from.clone()]
        } else {
            self.moves_from(from)
                .iter()
                .map(|next_pos| self.distinct_9_reachable_from(next_pos))
                .flatten()
                .unique()
                .collect()
        }
    }

    fn trailheads_rating(&self) -> u64 {
        self.trailheads()
            .iter()
            .map(|pos| self.count_paths_to_9(pos))
            .sum()
    }

    fn count_paths_to_9(&self, from: &(i32, i32)) -> u64 {
        if *self.cells.get(from).unwrap() == 9 {
            1
        } else {
            self.moves_from(from)
                .iter()
                .map(|next_pos| self.count_paths_to_9(next_pos))
                .sum()
        }
    }

    fn trailheads(&self) -> Vec<(i32, i32)> {
        self.cells
            .iter()
            .filter(|(pos, &height)| height == 0)
            .map(|(pos, _)| *pos)
            .collect()
    }

    fn moves_from(&self, from: &(i32, i32)) -> Vec<(i32, i32)> {
        let current_height = *self.cells.get(from).unwrap();
        [
            (from.0 + 1, from.1),
            (from.0 - 1, from.1),
            (from.0, from.1 + 1),
            (from.0, from.1 - 1),
        ]
        .iter()
        .filter(|pos| {
            self.cells
                .get(*pos)
                .filter(|&&adjacent_height| adjacent_height == current_height + 1)
                .is_some()
        })
        .map(|pos| *pos)
        .collect()
    }
}

fn part2(map: &TopoMap) -> u64 {
    map.trailheads_rating()
}
fn part1(map: &TopoMap) -> u64 {
    map.trailheads_score()
}

pub(crate) fn solve() {
    let content = fs::read_to_string("10.txt").unwrap();
    let map = TopoMap::new(&content);
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}
