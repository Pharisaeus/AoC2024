use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(PartialEq, Copy, Clone)]
enum CellType {
    Empty,
    Box,
    BoxL,
    BoxR,
    Wall,
}

impl CellType {
    fn new(c: &char) -> Self {
        match c {
            'O' => CellType::Box,
            '[' => CellType::BoxL,
            ']' => CellType::BoxR,
            '#' => CellType::Wall,
            _ => CellType::Empty,
        }
    }
}

struct Board {
    cells: HashMap<(i64, i64), CellType>,
    robot: (i64, i64),
}

impl Board {
    fn new(content: &str) -> Board {
        let mut cells = HashMap::new();
        let mut robot = (0, 0);
        for (y, line) in content.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    robot = (x as i64, y as i64);
                }
                cells.insert((x as i64, y as i64), CellType::new(&c));
            }
        }
        Board { cells, robot }
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.horizontal_step(direction),
            Direction::Right => self.horizontal_step(direction),
            Direction::Up => self.vertical_step(direction),
            Direction::Down => self.vertical_step(direction),
        }
    }

    fn horizontal_step(&mut self, direction: &Direction) {
        if self.horizontal_step_check(self.robot, direction) {
            let (x, y) = self.robot;
            let (dx, _) = direction.delta();
            self.make_horizontal_step((x + dx, y), direction, &CellType::Empty);
            self.robot = (self.robot.0 + dx, self.robot.1);
        }
    }

    fn horizontal_step_check(&self, (x, y): (i64, i64), direction: &Direction) -> bool {
        let (dx, _) = direction.delta();
        let next_cell = self.cells.get(&(x + dx, y)).unwrap();
        match next_cell {
            CellType::Wall => false,
            CellType::Empty => true,
            _ => self.horizontal_step_check((x + dx, y), direction),
        }
    }

    fn make_horizontal_step(
        &mut self,
        (x, y): (i64, i64),
        direction: &Direction,
        previous: &CellType,
    ) {
        let (dx, _) = direction.delta();
        let next_cell = *self.cells.get(&(x, y)).unwrap();
        match next_cell {
            CellType::Wall => return,
            CellType::Empty => {
                self.cells.insert((x, y), *previous);
            }
            next_type => {
                self.cells.insert((x, y), *previous);
                self.make_horizontal_step((x + dx, y), direction, &next_type);
            }
        }
    }

    fn vertical_step(&mut self, direction: &Direction) {
        if self.vertical_step_check(self.robot, direction) {
            let (x, y) = self.robot;
            let (_, dy) = direction.delta();
            self.make_vertical_step((x, y + dy), direction, &CellType::Empty);
            self.robot = (self.robot.0, self.robot.1 + dy);
        }
    }

    fn vertical_step_check(&self, (x, y): (i64, i64), direction: &Direction) -> bool {
        let (_, dy) = direction.delta();
        let next_cell = self.cells.get(&(x, y + dy)).unwrap();
        match next_cell {
            CellType::Wall => false,
            CellType::Empty => true,
            CellType::Box => self.vertical_step_check((x, y + dy), direction),
            CellType::BoxL => {
                self.vertical_step_check((x, y + dy), direction)
                    && return self.vertical_step_check((x + 1, y + dy), direction)
            }
            CellType::BoxR => {
                self.vertical_step_check((x, y + dy), direction)
                    && return self.vertical_step_check((x - 1, y + dy), direction)
            }
        }
    }

    fn make_vertical_step(
        &mut self,
        (x, y): (i64, i64),
        direction: &Direction,
        previous: &CellType,
    ) {
        let (_, dy) = direction.delta();
        let next_cell = self.cells.get(&(x, y)).unwrap();
        match next_cell {
            CellType::Wall => return,
            CellType::Empty => {
                self.cells.insert((x, y), *previous);
            }
            CellType::Box => {
                self.cells.insert((x, y), *previous);
                self.make_vertical_step((x, y + dy), direction, &CellType::Box);
            }
            CellType::BoxL => {
                self.cells.insert((x, y), *previous);
                self.cells.insert((x + 1, y), CellType::Empty);
                self.make_vertical_step((x, y + dy), direction, &CellType::BoxL);
                self.make_vertical_step((x + 1, y + dy), direction, &CellType::BoxR);
            }
            CellType::BoxR => {
                self.cells.insert((x, y), *previous);
                self.cells.insert((x - 1, y), CellType::Empty);
                self.make_vertical_step((x, y + dy), direction, &CellType::BoxR);
                self.make_vertical_step((x - 1, y + dy), direction, &CellType::BoxL);
            }
        }
    }

    fn is_cell(&self, current: &(i64, i64), target: &CellType) -> bool {
        self.cells.get(current).filter(|&c| c.eq(target)).is_some()
    }

    fn score(&self) -> i64 {
        self.cells
            .iter()
            .filter(|&(_, cell)| cell.eq(&CellType::Box) || cell.eq(&CellType::BoxL))
            .map(|((x, y), _)| x + 100 * y)
            .sum()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        let dim = *self.cells.keys().map(|(x, y)| y).max().unwrap();
        for y in 0..dim + 1 {
            for x in 0..(dim + 1) * 2 {
                if self.robot.eq(&(x, y)) {
                    res.push('@');
                } else if self.is_cell(&(x, y), &CellType::Empty) {
                    res.push('.');
                } else if self.is_cell(&(x, y), &CellType::Wall) {
                    res.push('#');
                } else if self.is_cell(&(x, y), &CellType::Box) {
                    res.push('0');
                } else if self.is_cell(&(x, y), &CellType::BoxL) {
                    res.push('[');
                } else if self.is_cell(&(x, y), &CellType::BoxR) {
                    res.push(']');
                }
            }
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn new(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!(),
        }
    }

    fn delta(&self) -> (i64, i64) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    fn label(&self) -> char {
        match self {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
}
fn solve_board(mut board: Board, directions: &Vec<Direction>) -> i64 {
    for d in directions {
        board.step(d);
    }
    println!("{}", board);
    board.score()
}
fn part2(b: &str, directions: &Vec<Direction>) -> i64 {
    let board_big = Board::new(
        b.replace('#', "##")
            .replace('O', "[]")
            .replace('.', "..")
            .replace('@', "@.")
            .as_str(),
    );
    solve_board(board_big, directions)
}

fn part1(b: &str, directions: &Vec<Direction>) -> i64 {
    let board = Board::new(b);
    solve_board(board, directions)
}
pub(crate) fn solve() {
    let content = fs::read_to_string("15.txt").unwrap();
    let (b, m) = content.split("\n\n").collect_tuple().unwrap();
    let directions = m
        .chars()
        .filter(|c| !c.eq(&'\n'))
        .map(Direction::new)
        .collect();
    println!("{}", part1(b, &directions));
    println!("{}", part2(b, &directions));
}
