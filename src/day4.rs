use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

struct Board {
    cells: HashMap<(i64, i64), char>,
    max_x: i64,
    max_y: i64,
}

impl Board {
    fn new(content: &str) -> Board {
        let max_x = content.len() as i64;
        let max_y = content.lines().count() as i64;
        Board {
            cells: content
                .lines()
                .enumerate()
                .map(|(row_index, row)| {
                    row.chars()
                        .enumerate()
                        .map(move |(col_index, c)| ((row_index as i64, col_index as i64), c))
                })
                .flatten()
                .collect(),
            max_x,
            max_y,
        }
    }

    fn count_xmas(&self) -> i64 {
        (0..self.max_x)
            .map(|x| (0..self.max_y).map(move |y| self.count_xmas_from_pos((x, y))))
            .flatten()
            .sum()
    }

    fn count_xmas_from_pos(&self, pos: (i64, i64)) -> i64 {
        self.sequences_from(pos)
            .iter()
            .filter(|seq| seq.as_str().eq("MAS"))
            .count() as i64
    }

    fn sequences_from(&self, pos: (i64, i64)) -> Vec<String> {
        let transformations = [
            Board::up,
            Board::down,
            Board::left,
            Board::right,
            Board::diag_up_right,
            Board::diag_down_right,
            Board::diag_up_left,
            Board::diag_down_left,
        ];
        match self.cells.get(&pos) {
            Some('X') => transformations
                .map(|f| f(self, pos, 3))
                .iter()
                .filter(|o| o.is_some())
                .map(|o| o.clone().unwrap())
                .collect_vec(),
            _ => vec![],
        }
    }

    fn count_mas(&self) -> i64 {
        (0..self.max_x)
            .map(|x| (0..self.max_y).map(move |y| self.is_mas_at_position((x, y))))
            .flatten()
            .filter(|&x| x)
            .count() as i64
    }

    fn is_mas_at_position(&self, pos: (i64, i64)) -> bool {
        match self.cells.get(&pos) {
            Some('A') => {
                let ul = self.diag_up_left(pos,1);
                let dr = self.diag_down_right(pos,1);
                match (ul.as_deref(), dr.as_deref()) {
                    (Some("M"), Some("S")) | (Some("S"), Some("M")) => {
                        let dl = self.diag_down_left(pos,1);
                        let ur = self.diag_up_right(pos,1);
                        match (dl.as_deref(), ur.as_deref()) {
                            (Some("M"), Some("S")) | (Some("S"), Some("M")) => true,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn up(&self, (x, y): (i64, i64), len: i64) -> Option<String> {
        self.extract_sequence((x, y), |(x, y), i| (x, y + i), len)
    }

    fn down(&self, (x, y): (i64, i64), len: i64) -> Option<String> {
        self.extract_sequence((x, y), |(x, y), i| (x, y - i), len)
    }

    fn left(&self, (x, y): (i64, i64), len: i64) -> Option<String> {
        self.extract_sequence((x, y), |(x, y), i| (x - i, y), len)
    }
    fn right(&self, (x, y): (i64, i64), len: i64) -> Option<String> {
        self.extract_sequence((x, y), |(x, y), i| (x + i, y), len)
    }

    fn diag_up_right(&self, (x, y): (i64, i64), len: i64) -> Option<String> {
        self.extract_sequence((x, y), |(x, y), i| (x + i, y + i), len)
    }

    fn diag_down_right(&self, (x, y): (i64, i64), len: i64) -> Option<String> {
        self.extract_sequence((x, y), |(x, y), i| (x + i, y - i), len)
    }

    fn diag_up_left(&self, (x, y): (i64, i64), len: i64) -> Option<String> {
        self.extract_sequence((x, y), |(x, y), i| (x - i, y + i), len)
    }

    fn diag_down_left(&self, (x, y): (i64, i64), len: i64) -> Option<String> {
        self.extract_sequence((x, y), |(x, y), i| (x - i, y - i), len)
    }

    fn extract_sequence(
        &self,
        pos: (i64, i64),
        step: fn((i64, i64), i64) -> (i64, i64),
        len: i64,
    ) -> Option<String> {
        let sequence = (1..=len)
            .map(|i| self.cells.get(&step(pos, i)))
            .collect_vec();
        if sequence.iter().all(Option::is_some) {
            Some(sequence.iter().map(|x| x.unwrap()).join(""))
        } else {
            None
        }
    }
}
fn part2(board: &Board) -> i64 {
    board.count_mas()
}
fn part1(board: &Board) -> i64 {
    board.count_xmas()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("4.txt").unwrap();
    let board = Board::new(&contents);
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
