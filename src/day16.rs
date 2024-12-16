use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;

#[derive(PartialEq)]
enum CellType {
    Empty,
    Wall,
}

impl CellType {
    fn new(c: &char) -> CellType {
        match c {
            '#' => CellType::Wall,
            &_ => CellType::Empty,
        }
    }
}
struct Maze {
    cells: HashMap<(i64, i64), CellType>,
    start: (i64, i64),
    end: (i64, i64),
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Vertex {
    pos: (i64, i64),
    d: Direction,
}

impl Maze {
    fn new(content: &str) -> Self {
        let mut cells = HashMap::new();
        let mut start: (i64, i64) = (0, 0);
        let mut end: (i64, i64) = (0, 0);
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

    fn bellman_ford(&self) -> (i64, HashSet<Vertex>) {
        let mut costs = HashMap::new();
        let mut pred = HashMap::new();
        for &(i, j) in self.cells.keys() {
            for d in [Direction::N, Direction::S, Direction::E, Direction::W] {
                costs.insert(Vertex { pos: (i, j), d }, 999999999999);
                pred.insert(Vertex { pos: (i, j), d }, HashSet::new());
            }
        }
        costs.insert(
            Vertex {
                pos: self.start,
                d: Direction::E,
            },
            0,
        );
        for _ in 0..self.cells.keys().count() {
            let mut relaxations = 0;
            for &(ux, uy) in self.cells.keys() {
                for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
                    let u = Vertex {
                        pos: (ux, uy),
                        d: dir,
                    };
                    for (v, weight) in self.edges(u) {
                        if self
                            .cells
                            .get(&v.pos)
                            .filter(|&c| c != &CellType::Wall)
                            .is_some()
                        {
                            let relaxed = costs.get(&u).unwrap() + weight;
                            let current_cost = *costs.get(&v).unwrap();
                            if current_cost > relaxed {
                                relaxations += 1;
                                costs.insert(v, relaxed);
                                let mut predecessors = HashSet::new();
                                predecessors.insert(u);
                                pred.insert(v, predecessors);
                            } else if current_cost == relaxed {
                                pred.get_mut(&v).unwrap().insert(u);
                            }
                        }
                    }
                }
            }
            if relaxations == 0 {
                break;
            }
        }
        let (cost, last_direction) = [Direction::N, Direction::S, Direction::E, Direction::W]
            .map(|d| Vertex { pos: self.end, d })
            .iter()
            .filter(|k| pred.contains_key(k))
            .map(|k| (*costs.get(k).unwrap(), k.d))
            .min_by(|x, y| x.0.cmp(&y.0))
            .unwrap();
        (cost, self.backtrace(pred, last_direction))
    }
    fn edges(&self, u: Vertex) -> Vec<(Vertex, i64)> {
        let d = u.d;
        let (dx, dy) = d.deltas();
        let (x, y) = u.pos;
        vec![
            (
                Vertex {
                    pos: (x + dx, y + dy),
                    d,
                },
                1,
            ),
            (
                Vertex {
                    pos: (x, y),
                    d: d.clockwise(),
                },
                1000,
            ),
            (
                Vertex {
                    pos: (x, y),
                    d: d.counter_clockwise(),
                },
                1000,
            ),
        ]
    }

    fn backtrace(&self, pred: HashMap<Vertex, HashSet<Vertex>>, d: Direction) -> HashSet<Vertex> {
        let mut res = HashSet::new();
        let mut to_check = VecDeque::new();
        let mut current = Vertex { pos: self.end, d };
        to_check.push_back(current);
        while !to_check.is_empty() {
            current = to_check.pop_front().unwrap();
            res.insert(current);
            let predecessors = pred.get(&current).unwrap();
            for v in predecessors {
                if !res.contains(v) {
                    to_check.push_back(*v);
                }
            }
        }
        res
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn deltas(&self) -> (i64, i64) {
        match self {
            Direction::N => (0, 1),
            Direction::S => (0, -1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
        }
    }

    fn clockwise(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::E => Direction::S,
            Direction::W => Direction::N,
        }
    }

    fn counter_clockwise(&self) -> Direction {
        self.clockwise().clockwise().clockwise()
    }
}
pub(crate) fn solve() {
    let content = fs::read_to_string("16.txt").unwrap();
    let maze = Maze::new(&content);
    let (cost, path) = maze.bellman_ford();
    println!("{}", cost);
    println!("{}", path.iter().map(|&v| v.pos).unique().count());
}
