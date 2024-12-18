use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::fs;

#[derive(Clone)]
enum Block {
    Empty,
    ID(u64),
}

impl Block {
    fn value(&self) -> u64 {
        match self {
            Block::Empty => 0,
            Block::ID(id) => *id,
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Block::Empty => ".".to_string(),
            Block::ID(id) => id.to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone)]
struct Disk {
    blocks: Vec<Block>,
    file_meta: HashMap<u64, (usize, usize)>,
}

impl Disk {
    fn new(content: &str) -> Self {
        let mut blocks = Vec::new();
        let mut file_meta: HashMap<u64, (usize, usize)> = HashMap::new();
        let mut current_is_empty = false;
        let mut block_id = 0;
        let mut current_index = 0;
        for c in content.chars() {
            let number_of_blocks = c.to_string().as_str().parse().unwrap();
            for _ in 0..number_of_blocks {
                if current_is_empty {
                    blocks.push(Block::Empty);
                } else {
                    blocks.push(Block::ID(block_id));
                }
                current_index += 1;
            }
            if !current_is_empty {
                let file_start = current_index - number_of_blocks;
                file_meta.insert(block_id, (file_start, number_of_blocks));
                block_id += 1;
            }
            current_is_empty = !current_is_empty;
        }
        Self { blocks, file_meta }
    }

    fn defrag_blocks(&mut self) {
        let mut empty_indices = VecDeque::new();
        let mut taken_indices = VecDeque::new();
        for (index, block) in self.blocks.iter().enumerate() {
            match block {
                Block::Empty => empty_indices.push_back(index),
                Block::ID(_) => taken_indices.push_back(index),
            }
        }
        loop {
            let first_empty = empty_indices.pop_front().unwrap();
            let last_taken = taken_indices.pop_back().unwrap();
            if first_empty < last_taken {
                self.blocks[first_empty] = self.blocks[last_taken].clone();
                self.blocks[last_taken] = Block::Empty;
            } else {
                break;
            }
        }
    }

    fn defrag_files(&mut self) {
        let mut empty_segments = self.find_empty_segments();
        let max_file_id = *self.file_meta.keys().max().unwrap();
        for current_file_id in (0_u64..=max_file_id).rev() {
            self.defrag_file(current_file_id, &mut empty_segments);
        }
    }

    fn defrag_file(&mut self, current_file_id: u64, empty_segments: &mut Vec<(usize, usize)>) {
        let (file_start, file_size) = *self.file_meta.get(&current_file_id).unwrap();
        for (empty_index, (empty_start, empty_size)) in empty_segments.iter().enumerate() {
            if file_size <= *empty_size && file_start > *empty_start {
                self.delete_file(current_file_id);
                self.insert_file(current_file_id, *empty_start);
                let remaining = empty_size - file_size;
                if remaining > 0 {
                    let new_empty_block_start = *empty_start + file_size;
                    empty_segments[empty_index] = (new_empty_block_start, remaining);
                } else {
                    empty_segments.remove(empty_index);
                }
                break;
            }
        }
    }

    fn find_empty_segments(&self) -> Vec<(usize, usize)> {
        let mut empty_segments = vec![];
        let mut segment_size = 0usize;
        let mut segment_start = 0usize;
        for (index, block) in self.blocks.iter().enumerate() {
            match block {
                Block::Empty => {
                    if segment_size == 0 {
                        segment_start = index;
                    }
                    segment_size += 1
                }
                Block::ID(_) => {
                    if segment_size > 0 {
                        empty_segments.push((segment_start, segment_size));
                        segment_size = 0;
                    }
                }
            }
        }
        empty_segments
    }

    fn delete_file(&mut self, current_file_id: u64) {
        let (start, size) = self.file_meta.get(&current_file_id).unwrap();
        for i in 0..*size {
            self.blocks[start + i] = Block::Empty;
        }
    }

    fn insert_file(&mut self, current_file_id: u64, new_start_index: usize) {
        let (_, current_file_size) = self.file_meta.get(&current_file_id).unwrap();
        for part in 0..*current_file_size {
            self.blocks[new_start_index + part] = Block::ID(current_file_id);
        }
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(idx, block)| idx as u64 * block.value())
            .sum()
    }
}

fn part2(disk: &Disk) -> u64 {
    let mut disk = disk.clone();
    disk.defrag_files();
    disk.checksum()
}
fn part1(disk: &Disk) -> u64 {
    let mut disk = disk.clone();
    disk.defrag_blocks();
    disk.checksum()
}

pub(crate) fn solve() {
    let content = fs::read_to_string("9.txt").unwrap();
    let disk = Disk::new(&content);
    println!("{}", part1(&disk));
    println!("{}", part2(&disk));
}
