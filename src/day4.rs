use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

struct Board {
    cells: HashMap<(i64, i64), char>
}

impl Board {
    fn new(content: &str) -> Board {
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
                .collect()
        }
    }

    fn count_xmas(&self) -> usize {
        self.cells.keys()
            .map(|&pos| self.count_sequences_from(pos))
            .sum()
    }

    fn count_sequences_from(&self, pos: (i64, i64)) -> usize {
        let steps = [
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
            Some('X') => steps
                .map(|f| f(self, pos, 3))
                .iter()
                .filter(|o| o.as_deref().eq(&Some("MAS")))
                .count(),
            _ => 0,
        }
    }

    fn count_mas(&self) -> usize {
        self.cells.keys()
            .filter(|&&pos| self.is_mas_at_position(pos))
            .count()
    }

    fn is_mas_at_position(&self, pos: (i64, i64)) -> bool {
        match self.cells.get(&pos) {
            Some('A') => {
                let ul = self.diag_up_left(pos, 1);
                let dr = self.diag_down_right(pos, 1);
                match (ul.as_deref(), dr.as_deref()) {
                    (Some("M"), Some("S")) | (Some("S"), Some("M")) => {
                        let dl = self.diag_down_left(pos, 1);
                        let ur = self.diag_up_right(pos, 1);
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
fn part2(board: &Board) -> usize {
    board.count_mas()
}
fn part1(board: &Board) -> usize {
    board.count_xmas()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("4.txt").unwrap();
    let board = Board::new(&contents);
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
