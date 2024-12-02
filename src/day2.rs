use std::fs;

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn new(line: &str) -> Report {
        Report {
            levels: line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
        }
    }

    fn is_safe(&self) -> bool {
        self.is_close() && (self.is_increasing() || self.is_decreasing())
    }

    fn is_safe_omit_one(&self) -> bool {
        for (index, _) in self.levels.iter().enumerate() {
            let mut levels_omit_one = self.levels.clone();
            levels_omit_one.remove(index);
            let report = Report {
                levels: levels_omit_one,
            };
            if report.is_safe() {
                return true;
            }
        }
        false
    }

    fn is_increasing(&self) -> bool {
        self.levels.windows(2).all(|w| w[0] < w[1])
    }

    fn is_decreasing(&self) -> bool {
        self.levels.windows(2).all(|w| w[0] > w[1])
    }

    fn is_close(&self) -> bool {
        self.levels
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]))
            .all(|diff| diff >= 1 && diff <= 3)
    }
}

fn part2(reports: &Vec<Report>) -> usize {
    reports.iter().filter(|x| x.is_safe_omit_one()).count()
}

fn part1(reports: &Vec<Report>) -> usize {
    reports.iter().filter(|x| x.is_safe()).count()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("2.txt").unwrap();
    let reports = contents.lines().map(Report::new).collect();
    println!("{}", part1(&reports));
    println!("{}", part2(&reports));
}
