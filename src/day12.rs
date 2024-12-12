use crate::day12::EdgeDirection::{Horizontal, Vertical};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;

struct Board {
    cells: HashMap<(i64, i64), char>,
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
                        .map(move |(col_index, c)| ((col_index as i64, row_index as i64), c))
                })
                .flatten()
                .collect(),
        }
    }
    fn find_regions(&self) -> Vec<Region> {
        let mut visited = HashSet::new();
        let mut regions = Vec::new();
        for pos in self.cells.keys() {
            if !visited.contains(pos) {
                let point = Point { x: pos.0, y: pos.1 };
                let label = self.cells[pos];
                let points = bfs(&point, |p| self.neighbours(p));
                visited.extend(points.iter().map(|p| (p.x, p.y)));
                let region = Region { label, points };
                regions.push(region);
            }
        }
        regions
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let x = point.x;
        let y = point.y;
        let label = self.cells[&(x, y)];
        vec![(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
            .iter()
            .filter(|pos| self.cells.contains_key(pos))
            .filter(|pos| self.cells[pos] == label)
            .map(|p| Point { x: p.0, y: p.1 })
            .collect()
    }
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum EdgeDirection {
    Horizontal,
    Vertical,
}

impl EdgeDirection {
    fn deltas(&self) -> Vec<(i64, i64)> {
        match self {
            Horizontal => vec![(-1, 0), (1, 0)],
            Vertical => vec![(0, -1), (0, 1)],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum EdgeLocation {
    Left,
    Right,
    Up,
    Down,
}

impl EdgeLocation {
    fn deltas(&self) -> ((i64, i64), (i64, i64)) {
        match self {
            EdgeLocation::Left => ((0, 0), (0, 1)),
            EdgeLocation::Right => ((1, 0), (1, 1)),
            EdgeLocation::Up => ((0, 1), (1, 1)),
            EdgeLocation::Down => ((0, 0), (1, 0)),
        }
    }

    fn edge_for_point(&self, point: &Point) -> Edge {
        let (x, y) = (point.x, point.y);
        let ((first_dx, first_dy), (second_dx, second_dy)) = self.deltas();
        Edge {
            first: Point {
                x: x + first_dx,
                y: y + first_dy,
            },
            second: Point {
                x: x + second_dx,
                y: y + second_dy,
            },
            location: self.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    first: Point,
    second: Point,
    location: EdgeLocation,
}

impl Edge {
    fn direction(&self) -> EdgeDirection {
        if self.first.x == self.second.x {
            Vertical
        } else {
            Horizontal
        }
    }
}

impl Point {
    fn surround_edges(&self) -> Vec<Edge> {
        [
            EdgeLocation::Left,
            EdgeLocation::Right,
            EdgeLocation::Up,
            EdgeLocation::Down,
        ]
        .map(|location| location.edge_for_point(self))
        .iter()
        .map(|e| *e)
        .collect()
    }
}
struct Region {
    label: char,
    points: Vec<Point>,
}

impl Region {
    fn area(&self) -> i64 {
        self.points.len() as i64
    }

    fn perimeter_edges(&self) -> HashSet<Edge> {
        let single_edges: HashSet<(Point, Point)> = self
            .points
            .iter()
            .map(|point| point.surround_edges())
            .flatten()
            .map(|edge| (edge.first, edge.second))
            .counts()
            .iter()
            .filter(|(_points, count)| **count == 1)
            .map(|(points, _count)| *points)
            .collect();
        self.points
            .iter()
            .map(|point| point.surround_edges())
            .flatten()
            .filter(|e| single_edges.contains(&(e.first, e.second)))
            .collect()
    }

    fn perimeter(&self) -> i64 {
        self.perimeter_edges().len() as i64
    }

    fn sides(&self) -> i64 {
        let edges = self.perimeter_edges();
        let mut sides = 0;
        let mut visited_points = HashSet::new();
        for e in &edges {
            let edge_direction = e.direction();
            for p in [e.first, e.second] {
                if !visited_points.contains(&(p, edge_direction)) {
                    let side = bfs(&p, |p| {
                        self.neighbours(p, &edge_direction, &e.location, &edges)
                    });
                    sides += 1;
                    for side_point in side {
                        visited_points.insert((side_point, edge_direction));
                    }
                }
            }
        }
        sides
    }

    fn neighbours(
        &self,
        current: &Point,
        edge_direction: &EdgeDirection,
        edge_location: &EdgeLocation,
        perimeter_edges: &HashSet<Edge>,
    ) -> Vec<Point> {
        let deltas = edge_direction.deltas();
        let mut neighbours = vec![];
        for (dx, dy) in deltas.iter() {
            let neighbour = Point {
                x: current.x + dx,
                y: current.y + dy,
            };
            let e1 = Edge {
                first: *current,
                second: neighbour,
                location: *edge_location,
            };
            let e2 = Edge {
                first: neighbour,
                second: *current,
                location: *edge_location,
            };
            if perimeter_edges.contains(&e1) || perimeter_edges.contains(&e2) {
                neighbours.push(neighbour);
            }
        }
        neighbours
    }

    fn fence_price(&self) -> i64 {
        self.area() * self.perimeter()
    }

    fn bulk_fence_price(&self) -> i64 {
        self.area() * self.sides()
    }
}

fn part2(regions: &Vec<Region>) -> i64 {
    regions.iter().map(|region| region.bulk_fence_price()).sum()
}

fn part1(regions: &Vec<Region>) -> i64 {
    regions.iter().map(|region| region.fence_price()).sum()
}

fn bfs<T: Clone + Copy + Hash + Eq>(start: &T, neighbours: impl Fn(&T) -> Vec<T>) -> Vec<T> {
    let mut result = vec![];
    let mut visited = HashSet::new();
    let mut to_check = VecDeque::new();
    to_check.push_back(*start);
    while !to_check.is_empty() {
        let current = to_check.pop_front().unwrap();
        if !visited.contains(&current) {
            visited.insert(current);
            result.push(current);
            for neighbour in neighbours(&current) {
                to_check.push_back(neighbour);
            }
        }
    }
    result
}

pub(crate) fn solve() {
    let content = fs::read_to_string("12.txt").unwrap();
    let board = Board::new(&content);
    let regions = board.find_regions();
    println!("{}", part1(&regions));
    println!("{}", part2(&regions));
}
