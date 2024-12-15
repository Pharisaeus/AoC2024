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

    fn step_big(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.step_simple(direction),
            Direction::Right => self.step_simple(direction),
            Direction::Up => self.big_step(direction),
            Direction::Down => self.big_step(direction),
        }
    }

    fn step_simple(&mut self, direction: &Direction) {
        if self.simple_check_step(direction) {
            let (dx, dy) = direction.delta();
            let mut current = self.robot;
            let mut previous = self.cells.get(&current).unwrap().clone();
            self.robot = (self.robot.0 + dx, self.robot.1 + dy);
            loop {
                current.0 += dx;
                current.1 += dy;
                let tmp = self.cells.get(&current).unwrap().clone();
                self.cells.insert(current, previous.clone());
                previous = tmp;
                if previous.eq(&CellType::Empty) {
                    break;
                }
            }
        }
    }

    fn simple_check_step(&self, direction: &Direction) -> bool {
        let (dx, dy) = direction.delta();
        let mut current = self.robot;
        current.0 += dx;
        current.1 += dy;
        loop {
            if self.is_cell(&current, &CellType::Empty) {
                return true;
            } else if self.is_cell(&current, &CellType::Wall) {
                return false;
            } else {
                current.0 += dx;
                current.1 += dy;
            }
        }
    }

    fn big_step(&mut self, direction: &Direction) {
        if self.big_step_check(self.robot, direction) {
            let (x, y) = self.robot;
            let (_, dy) = direction.delta();
            self.make_big_step((x, y + dy), direction, &CellType::Empty);
            self.robot = (self.robot.0, self.robot.1 + dy);
        }
    }

    fn make_big_step(&mut self, (x, y): (i64, i64), direction: &Direction, previous: &CellType) {
        let (_, dy) = direction.delta();
        let next_cell = self.cells.get(&(x, y)).unwrap();
        match next_cell {
            CellType::Box => {
                panic!()
            }
            CellType::Wall => return,
            CellType::Empty => {
                self.cells.insert((x, y), *previous);
            }
            CellType::BoxL => {
                self.cells.insert((x, y), *previous);
                self.cells.insert((x + 1, y), CellType::Empty);
                self.make_big_step((x, y + dy), direction, &CellType::BoxL);
                self.make_big_step((x + 1, y + dy), direction, &CellType::BoxR);
            }
            CellType::BoxR => {
                self.cells.insert((x, y), *previous);
                self.cells.insert((x - 1, y), CellType::Empty);
                self.make_big_step((x, y + dy), direction, &CellType::BoxR);
                self.make_big_step((x - 1, y + dy), direction, &CellType::BoxL);
            }
        }
    }

    fn big_step_check(&self, (x, y): (i64, i64), direction: &Direction) -> bool {
        let (_, dy) = direction.delta();
        let next_cell = self.cells.get(&(x, y + dy)).unwrap();
        match next_cell {
            CellType::Box => {
                panic!()
            }
            CellType::Wall => false,
            CellType::Empty => true,
            CellType::BoxL => {
                self.big_step_check((x, y + dy), direction)
                    && return self.big_step_check((x + 1, y + dy), direction)
            }
            CellType::BoxR => {
                self.big_step_check((x, y + dy), direction)
                    && return self.big_step_check((x - 1, y + dy), direction)
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

fn part2(mut board: Board, directions: &Vec<Direction>) -> i64 {
    for d in directions {
        board.step_big(d);
    }
    println!("{}", board);
    board.score()
}

fn part1(mut board: Board, directions: &Vec<Direction>) -> i64 {
    for d in directions {
        board.step_simple(d);
    }
    println!("{}", board);
    board.score()
}
pub(crate) fn solve() {
    let content = fs::read_to_string("15.txt").unwrap();
    let (b, m) = content.split("\n\n").collect_tuple().unwrap();
    let board = Board::new(b);
    let directions = m
        .chars()
        .filter(|c| !c.eq(&'\n'))
        .map(Direction::new)
        .collect();
    println!("{}", part1(board, &directions));
    let board_big = Board::new(
        b.replace('#', "##")
            .replace('O', "[]")
            .replace('.', "..")
            .replace('@', "@.")
            .as_str(),
    );
    println!("{}", part2(board_big, &directions));
}
