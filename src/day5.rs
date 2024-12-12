use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;

struct OrderingRule {
    first: u32,
    second: u32,
}

impl OrderingRule {
    fn new(line: &str) -> Self {
        let (first, second) = line
            .split("|")
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { first, second }
    }

    fn is_valid(&self, x: &u32, y: &u32) -> bool {
        !(&self.first == y && &self.second == x)
    }
}

struct OrderingRules {
    rules: Vec<OrderingRule>,
}

impl OrderingRules {
    fn new(content: &str) -> Self {
        let rules_segment = content
            .split("\n\n")
            .find_or_first(|_x| true)
            .unwrap();
        Self {
            rules: rules_segment
                .lines()
                .map(|line| OrderingRule::new(line))
                .collect(),
        }
    }
    fn is_valid_pair(&self, x: &u32, y: &u32) -> bool {
        self.rules.iter().all(|rule| rule.is_valid(x, y))
    }
    fn is_valid(&self, update: &Update) -> bool {
        update
            .pages
            .windows(2)
            .all(|x|self.is_valid_pair(&x[0],&x[1]))
    }

    fn cmp(&self, x: &u32, y: &u32) -> Ordering {
        match self.is_valid_pair(x, y) {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }

    fn sort(&self, update: &Update) -> Update {
        Update {
            pages: update
                .pages
                .iter()
                .sorted_by(|x, y| self.cmp(x, y))
                .map(|x| x.clone())
                .collect(),
        }
    }
}

struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn new(line: &str) -> Self {
        Self {
            pages: line.split(",").map(|page| page.parse().unwrap()).collect(),
        }
    }

    fn median(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }
}

struct Updates {
    updates: Vec<Update>,
}

impl Updates {
    fn new(content: &str) -> Self {
        let updates_segment = content
            .split("\n\n")
            .skip(1)
            .find_or_first(|_x| true)
            .unwrap();
        Self {
            updates: updates_segment
                .lines()
                .map(|line| Update::new(line))
                .collect(),
        }
    }
}

fn part2(ordering_rules: &OrderingRules, updates: &Updates) -> u32 {
    updates
        .updates
        .iter()
        .filter(|update| !ordering_rules.is_valid(update))
        .map(|update| ordering_rules.sort(update))
        .map(|update| update.median())
        .sum()
}

fn part1(ordering_rules: &OrderingRules, updates: &Updates) -> u32 {
    updates
        .updates
        .iter()
        .filter(|update| ordering_rules.is_valid(update))
        .map(|update| update.median())
        .sum()
}

pub(crate) fn solve() {
    let content = fs::read_to_string("5.txt").unwrap();
    let ordering_rules = OrderingRules::new(&content);
    let updates = Updates::new(&content);
    println!("{}", part1(&ordering_rules, &updates));
    println!("{}", part2(&ordering_rules, &updates));
}
