use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;
fn part2(corrupted_list: &Vec<(i64, i64)>, dimension: i64, start: usize) -> (i64, i64) {
    let mut last_path = find_path(corrupted_list, start, dimension).unwrap();
    for i in start..corrupted_list.len() {
        let new_block = corrupted_list[i];
        if last_path.contains(&new_block) {
            let new_path = find_path(corrupted_list, i + 1, dimension);
            match new_path {
                None => {
                    return new_block;
                }
                Some(path) => last_path = path,
            }
        }
    }
    panic!()
}

fn part1(corrupted_list: &Vec<(i64, i64)>, dimension: i64, start: usize) -> Option<i64> {
    let path = find_path(corrupted_list, start, dimension);
    path.map(|x| x.len() as i64 - 1)
}

fn find_path(
    corrupted_list: &Vec<(i64, i64)>,
    size: usize,
    dimension: i64,
) -> Option<HashSet<(i64, i64)>> {
    let corrupted: HashSet<&(i64, i64)> = corrupted_list.iter().take(size).collect();
    let path = bfs(&(0, 0), &(dimension, dimension), |&(x, y)| {
        let mut neighbours = vec![];
        for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if (nx, ny) != (x, y) && nx >= 0 && nx <= dimension && ny >= 0 && ny <= dimension {
                if !corrupted.contains(&(nx, ny)) {
                    neighbours.push((nx, ny));
                }
            }
        }
        neighbours
    });
    path
}

fn bfs<T: Clone + Copy + Hash + Eq>(
    start: &T,
    end: &T,
    neighbours: impl Fn(&T) -> Vec<T>,
) -> Option<HashSet<T>> {
    let mut pred = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_check = VecDeque::new();
    to_check.push_back(*start);
    while !to_check.is_empty() {
        let current = to_check.pop_front().unwrap();
        if current == *end {
            break;
        }
        if !visited.contains(&current) {
            visited.insert(current);
            for neighbour in neighbours(&current) {
                if !visited.contains(&neighbour) {
                    pred.insert(neighbour, current);
                }
                to_check.push_back(neighbour);
            }
        }
    }
    if !pred.contains_key(end) {
        None
    } else {
        let mut path = HashSet::new();
        let mut current = *end;
        while current != *start {
            path.insert(current);
            current = pred[&current];
        }
        path.insert(*start);
        Some(path)
    }
}
pub(crate) fn solve() {
    let content = fs::read_to_string("18.txt").unwrap();
    let corrupted = content
        .lines()
        .map(|line| line.split(",").collect_tuple().unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    println!("{:?}", part1(&corrupted, 70, 1024));
    println!("{:?}", part2(&corrupted, 70, 1024));
}
