use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

struct Board {
    cells: HashMap<(i32, i32), char>,
    antennas: HashMap<char, Vec<(i32, i32)>>,
    size: i32,
}

impl Board {
    fn new(content: &str) -> Self {
        let size = content.lines().count() as i32 - 1;
        let cells: HashMap<(i32, i32), char> = content
            .split('\n')
            .enumerate()
            .map(|(row_index, row)| {
                row.chars()
                    .map(|c| c)
                    .enumerate()
                    .map(move |(col_index, cell)| {
                        ((col_index as i32, size - row_index as i32), cell)
                    })
            })
            .flatten()
            .collect();
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for (pos, c) in cells.iter() {
            if c != &'.' {
                if !antennas.contains_key(c) {
                    antennas.insert(*c, Vec::new());
                }
                let mut v = antennas.get_mut(c).unwrap();
                v.push(*pos);
            }
        }
        Self {
            cells,
            antennas,
            size,
        }
    }

    fn count_antinodes(&self, any_position: bool) -> usize {
        self.antennas
            .keys()
            .map(|c| self.get_antinodes(c, any_position))
            .flatten()
            .filter(|pos| self.cells.contains_key(pos))
            .unique()
            .count()
    }

    fn get_antinodes(&self, c: &char, any_position: bool) -> Vec<(i32, i32)> {
        let nodes = self.antennas.get(c).unwrap();
        nodes
            .iter()
            .combinations(2)
            .map(|x| self.calculate_antinodes(*x[0], *x[1], any_position))
            .flatten()
            .unique()
            .collect()
    }

    fn calculate_antinodes(
        &self,
        (x1, y1): (i32, i32),
        (x2, y2): (i32, i32),
        any_position: bool,
    ) -> Vec<(i32, i32)> {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let multipliers = if !any_position { 1..=1 } else { 0..=self.size };
        multipliers
            .map(|m| [(x1 - dx * m, y1 - dy * m), (x2 + dx * m, y2 + dy * m)])
            .flatten()
            .collect()
    }
}
fn part2(board: &Board) -> usize {
    board.count_antinodes(true)
}

fn part1(board: &Board) -> usize {
    board.count_antinodes(false)
}

pub(crate) fn solve() {
    let content = fs::read_to_string("8.txt").unwrap();
    let board = Board::new(&content);
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
