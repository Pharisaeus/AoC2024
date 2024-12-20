use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs;
use std::ops::Sub;

#[derive(PartialEq)]
enum CellType {
    Empty,
    Wall,
}

impl CellType {
    fn new(c: &char) -> CellType {
        match c {
            '.' | 'S' | 'E' => CellType::Empty,
            '#' => CellType::Wall,
            _ => panic!(),
        }
    }
}
struct Board {
    cells: HashMap<(i64, i64), CellType>,
    start: (i64, i64),
    end: (i64, i64),
}

impl Board {
    fn new(content: &str) -> Self {
        let mut cells = HashMap::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in content.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (x as i64, y as i64);
                } else if c == 'E' {
                    end = (x as i64, y as i64);
                }
                cells.insert((x as i64, y as i64), CellType::new(&c));
            }
        }
        Self { cells, start, end }
    }

    fn distances(&self, from: (i64, i64)) -> HashMap<(i64, i64), i64> {
        let mut distances = HashMap::new();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        let mut to_check = VecDeque::new();
        to_check.push_back((from, 0));
        while !to_check.is_empty() {
            let (current, cost) = to_check.pop_front().unwrap();
            if !visited.contains(&current) {
                distances.insert(current, cost);
                visited.insert(current);
                for neighbour in self.neighbours(current) {
                    if !visited.contains(&neighbour) {
                        to_check.push_back((neighbour, cost + 1));
                    }
                }
            }
        }
        distances
    }

    fn adjacent(&self, (x, y): (i64, i64)) -> HashSet<(i64, i64)> {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .filter(|&&(x, y)| self.cells.contains_key(&(x, y)))
            .cloned()
            .collect()
    }
    fn neighbours(&self, (x, y): (i64, i64)) -> HashSet<(i64, i64)> {
        self.adjacent((x, y))
            .iter()
            .filter(|&&(x, y)| self.cells.get(&(x, y)) == Some(&CellType::Empty))
            .cloned()
            .collect()
    }

    fn far_neighbours(&self, (x, y): (i64, i64), duration: usize) -> HashSet<(i64, i64)> {
        let mut res = HashSet::new();
        let mut to_check = Vec::new();
        res.insert((x, y));
        to_check.push((x, y));
        for _ in 0..duration {
            let mut new_to_check = Vec::new();
            for current in to_check {
                for neighbour in self.adjacent(current) {
                    if !res.contains(&neighbour) {
                        res.insert(neighbour);
                        new_to_check.push(neighbour);
                    }
                }
            }
            to_check = new_to_check;
        }
        res.iter()
            .filter(|&&(x, y)| self.cells.get(&(x, y)) == Some(&CellType::Empty))
            .cloned()
            .collect()
    }

    fn dim(&self) -> i64 {
        *self.cells.keys().map(|(_, y)| y).max().unwrap()
    }

    fn manhattan(&self, (x1, y1): &(i64, i64), (x2, y2): &(i64, i64)) -> i64 {
        x1.sub(x2).abs() + y1.sub(y2).abs()
    }

    fn cheats(&self, duration: usize, cutoff: i64) -> HashSet<((i64, i64), (i64, i64))> {
        let distances_start = self.distances(self.start);
        let distances_end = self.distances(self.end);
        let regular = distances_start.get(&(self.end)).unwrap();
        let mut cheats = HashSet::new();
        for &cheat_start in self
            .cells
            .iter()
            .filter(|(pos, t)| t == &&CellType::Empty)
            .map(|(pos, _)| pos)
        {
            for cheat_end in self.far_neighbours(cheat_start, duration) {
                let distance_to_start = distances_start.get(&(cheat_start));
                let distance_to_end = distances_end.get(&(cheat_end));
                let manhattan = self.manhattan(&cheat_start, &cheat_end);
                match (distance_to_start, distance_to_end) {
                    (Some(ds), Some(de)) => {
                        let total = ds + de + manhattan;
                        let saved = regular - total;
                        if saved >= cutoff {
                            cheats.insert((cheat_start, cheat_end));
                        }
                    }
                    _ => {}
                }
            }
        }
        cheats
    }
}

fn part2(board: &Board) -> usize {
    board.cheats(20, 100).iter().count()
}
fn part1(board: &Board) -> usize {
    board.cheats(2, 100).iter().count()
}
pub(crate) fn solve() {
    let content = fs::read_to_string("20.txt").unwrap();
    let board = Board::new(&content);
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
