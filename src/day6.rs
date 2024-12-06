use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        }
    }

    fn step(&self, (x, y): (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.delta();
        (x + dx, y + dy)
    }
}

#[derive(PartialEq, Copy, Clone)]
enum CellType {
    Path,
    Obstacle,
    Start,
}

impl CellType {
    fn new(c: &char) -> Self {
        match c {
            '#' => CellType::Obstacle,
            '.' => CellType::Path,
            _ => CellType::Start,
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    pos: (i32, i32),
    direction: Direction,
}

#[derive(PartialEq)]
enum WalkState {
    OutOfBounds,
    Looped,
}

struct WalkResult {
    state: WalkState,
    visited_states: HashSet<State>,
}

impl WalkResult {
    fn visited_cells(&self) -> HashSet<(i32, i32)> {
        self.visited_states.iter().map(|state| state.pos).collect()
    }
}
struct Board {
    start: (i32, i32),
    cells: HashMap<(i32, i32), CellType>,
}

impl Board {
    fn new(content: &str) -> Self {
        let max_y = content.lines().count() as i32;
        let cells: HashMap<(i32, i32), CellType> = content
            .split('\n')
            .enumerate()
            .map(|(row_index, row)| {
                row.chars()
                    .map(|c| CellType::new(&c))
                    .enumerate()
                    .map(move |(col_index, cell)| {
                        ((col_index as i32, max_y - row_index as i32), cell)
                    })
            })
            .flatten()
            .collect();
        let start = cells
            .iter()
            .find_or_first(|&((x, y), c)| c.eq(&CellType::Start))
            .map(|(&(x, y), c)| (x, y))
            .unwrap();
        Self { start, cells }
    }

    fn is_legal(&self, pos: (i32, i32)) -> bool {
        self.cells
            .get(&pos)
            .filter(|&c| c != &CellType::Obstacle)
            .is_some()
    }

    fn is_oob(&self, pos: (i32, i32)) -> bool {
        !self.cells.contains_key(&pos)
    }
    fn propagate_guard(&self) -> WalkResult {
        let mut visited_states:HashSet<State> = HashSet::new();
        let mut current_cell = self.start;
        let mut current_direction = Direction::Up;
        loop {
            let current_state = State {
                pos: current_cell,
                direction: current_direction.clone(),
            };
            if visited_states.contains(&current_state) {
                return WalkResult {
                    state: WalkState::Looped,
                    visited_states,
                };
            }
            visited_states.insert(current_state);
            let next_cell = current_direction.step(current_cell);
            if self.is_oob(next_cell) {
                return WalkResult {
                    state: WalkState::OutOfBounds,
                    visited_states,
                };
            } else if self.is_legal(next_cell) {
                current_cell = next_cell
            } else {
                current_direction = current_direction.turn_right();
            }
        }
    }
}

fn part2(board: &Board) -> usize {
    let result = board.propagate_guard();
    let mut looped = 0;
    let mut base_board: HashMap<(i32, i32), CellType> =
        board.cells.iter().map(|(&pos, &c)| (pos, c)).collect();
    let mut cells_to_test = result.visited_cells();
    cells_to_test.remove(&board.start);
    for route_cell in cells_to_test {
        base_board.insert(route_cell, CellType::Obstacle);
        let new_board = Board {
            start: board.start,
            cells: base_board,
        };
        let result = new_board.propagate_guard();
        if result.state.eq(&WalkState::Looped) {
            looped += 1;
        }
        base_board = new_board.cells;
        base_board.insert(route_cell, CellType::Path);
    }
    looped
}

fn part1(board: &Board) -> usize {
    let result = board.propagate_guard();
    result.visited_cells().len()
}

pub(crate) fn solve() {
    let content = fs::read_to_string("6.txt").unwrap();
    let board = Board::new(&content);

    use std::time::Instant;
    let now = Instant::now();
    {
        println!("{}", part1(&board));
        println!("{}", part2(&board));
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
