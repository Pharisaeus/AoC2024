use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }

    fn to_key(self) -> Key {
        match self {
            Direction::UP => Key::UP,
            Direction::DOWN => Key::DOWN,
            Direction::LEFT => Key::LEFT,
            Direction::RIGHT => Key::RIGHT,
        }
    }

    fn vertical(dy: i64) -> Direction {
        if dy > 0 {
            Direction::DOWN
        } else {
            Direction::UP
        }
    }

    fn horizontal(dx: i64) -> Direction {
        if dx > 0 {
            Direction::RIGHT
        } else {
            Direction::LEFT
        }
    }
}
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Key {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    A,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Key {
    fn new(c: &char) -> Self {
        match c {
            '0' => Key::ZERO,
            '1' => Key::ONE,
            '2' => Key::TWO,
            '3' => Key::THREE,
            '4' => Key::FOUR,
            '5' => Key::FIVE,
            '6' => Key::SIX,
            '7' => Key::SEVEN,
            '8' => Key::EIGHT,
            '9' => Key::NINE,
            'A' => Key::A,
            '^' => Key::UP,
            'v' => Key::DOWN,
            '<' => Key::LEFT,
            '>' => Key::RIGHT,
            &_ => panic!(),
        }
    }

    fn index(&self) -> (i64, i64) {
        match self {
            Key::ZERO => (1, 3),
            Key::ONE => (0, 2),
            Key::TWO => (1, 2),
            Key::THREE => (2, 2),
            Key::FOUR => (0, 1),
            Key::FIVE => (1, 1),
            Key::SIX => (2, 1),
            Key::SEVEN => (0, 0),
            Key::EIGHT => (1, 0),
            Key::NINE => (2, 0),
            Key::A => (2, 3),
            Key::UP => (1, 3),
            Key::LEFT => (0, 4),
            Key::DOWN => (1, 4),
            Key::RIGHT => (2, 4),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Key::A => "A".to_string(),
            Key::UP => "^".to_string(),
            Key::DOWN => "v".to_string(),
            Key::LEFT => "<".to_string(),
            Key::RIGHT => ">".to_string(),
            _ => panic!(),
        }
    }
}

struct Code {
    code: Vec<Key>,
    numeric: i64,
}

impl Code {
    fn new(line: &str) -> Self {
        let code = line.chars().map(|ch| Key::new(&ch)).collect();
        let numeric = line
            .chars()
            .filter(|c| c.is_digit(10))
            .join("")
            .parse()
            .unwrap();
        Self { code, numeric }
    }
}

struct Keypad {
    connections: HashMap<(Key, Direction), Key>,
    paths: HashMap<(Key, Key), Vec<Direction>>,
}

impl Keypad {
    fn new(connections: &HashMap<(Key, Direction), Key>) -> Keypad {
        let mut bidirectional_connections = HashMap::new();
        for ((start, direction), end) in connections {
            bidirectional_connections.insert((*start, *direction), *end);
            bidirectional_connections.insert((*end, direction.opposite()), *start);
        }
        Keypad {
            paths: Self::compute_paths(&bidirectional_connections),
            connections: bidirectional_connections,
        }
    }

    fn compute_paths(
        connections: &HashMap<(Key, Direction), Key>,
    ) -> HashMap<(Key, Key), Vec<Direction>> {
        let mut paths: HashMap<(Key, Key), Vec<Direction>> = HashMap::new();
        for (start, _) in connections.keys() {
            paths.insert((*start, *start), vec![]);
        }
        for ((start, d), end) in connections {
            paths.insert((*start, *end), vec![*d]);
        }
        for start in connections.values() {
            for end in connections.values() {
                if !paths.contains_key(&(*start, *end)) {
                    let mut path = vec![];
                    let (sx, sy) = start.index();
                    let (ex, ey) = end.index();
                    let dx = ex - sx;
                    let dy = ey - sy;

                    if dx == 0 {
                        // only vertical
                        for _ in 0..dy.abs() {
                            path.push(Direction::vertical(dy))
                        }
                    } else if dy == 0 {
                        // only horizontal
                        for _ in 0..dx.abs() {
                            path.push(Direction::horizontal(dx));
                        }
                    } else {
                        // preference <^v>
                        if Direction::horizontal(dx) == Direction::LEFT {
                            // going left
                            if !(sy == 3 && ex == 0) {
                                // not hole
                                for _ in 0..dx.abs() {
                                    path.push(Direction::LEFT);
                                }
                                for _ in 0..dy.abs() {
                                    path.push(Direction::vertical(dy))
                                }
                            } else {
                                //avoid hole
                                for _ in 0..dy.abs() {
                                    path.push(Direction::vertical(dy))
                                }
                                for _ in 0..dx.abs() {
                                    path.push(Direction::LEFT);
                                }
                            }
                        } else {
                            //going right, so first do up/down unless down into hole
                            if !(sx == 0 && ey == 3) {
                                // not hole
                                for _ in 0..dy.abs() {
                                    path.push(Direction::vertical(dy))
                                }
                                for _ in 0..dx.abs() {
                                    path.push(Direction::horizontal(dx));
                                }
                            } else {
                                for _ in 0..dx.abs() {
                                    path.push(Direction::horizontal(dx));
                                }
                                for _ in 0..dy.abs() {
                                    path.push(Direction::vertical(dy))
                                }
                            }
                        }
                    }
                    paths.insert((*start, *end), path);
                }
            }
        }
        paths
    }
    fn find_sequence(&self, sequence: &Vec<Key>) -> Vec<Key> {
        let mut expanded = sequence.clone();
        expanded.insert(0, Key::A);
        expanded
            .windows(2)
            .map(|x| self.find_path(&x[0], &x[1]))
            .flatten()
            .collect()
    }

    fn find_path(&self, start: &Key, end: &Key) -> Vec<Key> {
        let mut res: Vec<Key> = self
            .paths
            .get(&(*start, *end))
            .unwrap()
            .iter()
            .map(|d| d.to_key())
            .collect();
        res.push(Key::A);
        res
    }
}

struct KeypadsSequence {
    keypads: Vec<Keypad>,
}
impl KeypadsSequence {
    fn new(keypads: Vec<Keypad>) -> KeypadsSequence {
        KeypadsSequence { keypads }
    }

    fn find_sequence(&mut self, sequence: &Vec<Key>) -> usize {
        // chunk from A to A
        // Group identical chunks
        // Only ask downstream keypad about each unique chunk once and multiply result by count
        let mut split_sequence = KeypadsSequence::split(sequence);
        for keypad in &self.keypads {
            let mut next_split_sequence = HashMap::new();
            for (sequence_chunk, count) in split_sequence.iter() {
                let new_sequence = keypad.find_sequence(sequence_chunk);
                let chunked = KeypadsSequence::split(&new_sequence);
                for (subsequence, how_many) in chunked {
                    let old = next_split_sequence.get(&subsequence).unwrap_or(&0);
                    next_split_sequence.insert(subsequence, old + how_many * count);
                }
            }
            split_sequence = next_split_sequence;
        }
        split_sequence
            .iter()
            .map(|(subsequence, how_many)| subsequence.len() * how_many)
            .sum()
    }

    fn split(sequence: &Vec<Key>) -> HashMap<Vec<Key>, usize> {
        // we can always chunk inputs from A to A
        let mut split = HashMap::new();
        let mut current = vec![];
        for k in sequence {
            current.push(k.clone());
            if k == &Key::A {
                let old = split.get(&current).unwrap_or(&0);
                split.insert(current.clone(), old + 1);
                current = vec![];
            }
        }
        split
    }
}

fn numeric_keypad() -> Keypad {
    let mut connections = HashMap::new();
    connections.insert((Key::ZERO, Direction::RIGHT), Key::A);
    connections.insert((Key::ZERO, Direction::UP), Key::TWO);
    connections.insert((Key::TWO, Direction::LEFT), Key::ONE);
    connections.insert((Key::TWO, Direction::RIGHT), Key::THREE);
    connections.insert((Key::TWO, Direction::UP), Key::FIVE);
    connections.insert((Key::FIVE, Direction::LEFT), Key::FOUR);
    connections.insert((Key::FIVE, Direction::RIGHT), Key::SIX);
    connections.insert((Key::FIVE, Direction::UP), Key::EIGHT);
    connections.insert((Key::EIGHT, Direction::LEFT), Key::SEVEN);
    connections.insert((Key::EIGHT, Direction::RIGHT), Key::NINE);
    Keypad::new(&connections)
}

fn directional_keypad() -> Keypad {
    let mut connections = HashMap::new();
    connections.insert((Key::DOWN, Direction::LEFT), Key::LEFT);
    connections.insert((Key::DOWN, Direction::RIGHT), Key::RIGHT);
    connections.insert((Key::DOWN, Direction::UP), Key::UP);
    connections.insert((Key::UP, Direction::RIGHT), Key::A);
    Keypad::new(&connections)
}

fn solution(codes: &Vec<Code>, depth: usize) -> i64 {
    let mut keypads_list: Vec<Keypad> = vec![numeric_keypad()];
    for _ in 0..depth {
        keypads_list.push(directional_keypad());
    }
    let mut keypads = KeypadsSequence::new(keypads_list);
    codes
        .iter()
        .map(|code| code.numeric * keypads.find_sequence(&code.code) as i64)
        .sum()
}

fn part2(codes: &Vec<Code>) -> i64 {
    solution(codes, 25)
}
fn part1(codes: &Vec<Code>) -> i64 {
    solution(codes, 2)
}
pub(crate) fn solve() {
    let content = fs::read_to_string("21.txt").unwrap();
    let codes = content.lines().map(|line| Code::new(line)).collect();
    println!("{}", part1(&codes));
    println!("{}", part2(&codes));
}
