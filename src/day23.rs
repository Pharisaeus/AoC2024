use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

struct Lan {
    connections: HashMap<String, HashSet<String>>,
}

impl Lan {
    fn new(content: &str) -> Self {
        let mut connections = HashMap::new();
        for line in content.lines() {
            let (a, b) = line.split("-").collect_tuple().unwrap();
            if !connections.contains_key(a) {
                connections.insert(a.to_string(), HashSet::new());
            }
            connections.get_mut(a).unwrap().insert(b.to_string());
            if !connections.contains_key(b) {
                connections.insert(b.to_string(), HashSet::new());
            }
            connections.get_mut(b).unwrap().insert(a.to_string());
        }
        Self { connections }
    }

    fn find_3_cliques(&self) -> HashSet<Clique> {
        let mut current = HashSet::new();
        for (a, n) in &self.connections {
            let c = Clique {
                nodes: vec![a.clone()],
                intersections: n.iter().cloned().collect(),
            };
            current.insert(c);
        }
        for _ in 0..2 {
            current = self.extend_cliques(&current)
        }
        current
    }

    fn extend_cliques(&self, last: &HashSet<Clique>) -> HashSet<Clique> {
        let mut res = HashSet::new();
        for c in last {
            res.extend(self.expand(&c))
        }
        res
    }

    fn find_max_clique(&self) -> Clique {
        let mut current = self.find_3_cliques();
        loop {
            current = self.extend_cliques(&current);
            if current.len() == 1 {
                return current.into_iter().next().unwrap();
            }
        }
    }

    fn expand(&self, previous: &Clique) -> HashSet<Clique> {
        let mut res = HashSet::new();
        for node in previous.intersections.iter() {
            let neighbours = self.connections.get(node);
            match neighbours {
                Some(nn) => {
                    let new_intersection =
                        HashSet::from_iter(previous.intersections.iter().cloned())
                            .intersection(nn)
                            .cloned()
                            .sorted()
                            .collect();
                    let mut new_clique = previous.nodes.clone();
                    new_clique.push(node.to_string());
                    new_clique.sort_unstable();
                    res.insert(Clique {
                        nodes: new_clique,
                        intersections: new_intersection,
                    });
                }
                None => {}
            }
        }
        res
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Clique {
    nodes: Vec<String>,
    intersections: Vec<String>,
}

fn part2(lan: &Lan) -> String {
    lan.find_max_clique().nodes.join(",")
}

fn part1(lan: &Lan) -> i64 {
    let cliques = lan.find_3_cliques();
    cliques
        .iter()
        .filter(|c| c.nodes.iter().any(|node| node.starts_with('t')))
        .count() as i64
}
pub(crate) fn solve() {
    let content = fs::read_to_string("23.txt").unwrap();
    let lan = Lan::new(&content);
    println!("{}", part1(&lan));
    println!("{}", part2(&lan));
}
