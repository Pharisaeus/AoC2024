use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    OR,
    AND,
    XOR,
}

impl Operation {
    fn evaluate(&self, inputs: &Vec<u8>) -> u8 {
        match self {
            Operation::OR => inputs.iter().cloned().reduce(|x, y| (x | y)).unwrap(),
            Operation::AND => inputs.iter().cloned().reduce(|x, y| (x & y)).unwrap(),
            Operation::XOR => inputs.iter().cloned().reduce(|x, y| (x ^ y)).unwrap(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct GateLabel {
    name: String,
    index: i64,
}

impl GateLabel {
    fn new(data: &str) -> Self {
        let index = data[1..].parse::<i64>().unwrap_or_else(|_| -1);
        Self {
            name: data.to_string(),
            index,
        }
    }

    fn is_xyz(&self, prefix: &str) -> bool {
        self.name.contains(prefix) && self.index > 0
    }
    fn is_x(&self) -> bool {
        self.is_xyz("x")
    }
    fn is_y(&self) -> bool {
        self.is_xyz("y")
    }
    fn is_z(&self) -> bool {
        self.is_xyz("z")
    }
}

#[derive(Debug)]
struct Gate {
    inputs: Vec<GateLabel>,
    operation: Operation,
    output: GateLabel,
    result: Option<u8>,
}

impl Gate {
    fn new(line: &str) -> Self {
        let (connections, result) = line.split(" -> ").collect_tuple().unwrap();
        let mut inputs = vec![];
        let mut operation = Operation::XOR;
        if connections.contains(" AND ") {
            let (a, b) = connections.split(" AND ").collect_tuple().unwrap();
            operation = Operation::AND;
            inputs.push(GateLabel::new(a));
            inputs.push(GateLabel::new(b));
        } else if connections.contains(" OR ") {
            let (a, b) = connections.split(" OR ").collect_tuple().unwrap();
            operation = Operation::OR;
            inputs.push(GateLabel::new(a));
            inputs.push(GateLabel::new(b));
        } else {
            let (a, b) = connections.split(" XOR ").collect_tuple().unwrap();
            operation = Operation::XOR;
            inputs.push(GateLabel::new(a));
            inputs.push(GateLabel::new(b));
        }
        Self {
            inputs,
            operation,
            output: GateLabel::new(result),
            result: None,
        }
    }

    fn evaluate(&mut self, context: &HashMap<String, u8>) -> Option<u8> {
        match self.result {
            None => {
                if self.inputs.iter().all(|x| context.contains_key(&x.name)) {
                    let values = self
                        .inputs
                        .iter()
                        .map(|x| *context.get(&x.name).unwrap())
                        .collect();
                    self.result = Some(self.operation.evaluate(&values));
                    self.result
                } else {
                    None
                }
            }
            Some(_) => self.result,
        }
    }
}
struct Wires {
    values: HashMap<String, u8>,
    connections: HashMap<GateLabel, Gate>,
}

impl Wires {
    fn new(content: &str) -> Self {
        let (initial, connection) = content.split("\n\n").collect_tuple().unwrap();
        let mut values = HashMap::new();
        for line in initial.lines() {
            let (label, value) = line.split(": ").collect_tuple().unwrap();
            values.insert(label.to_string(), value.parse().unwrap());
        }
        let mut connections = connection
            .lines()
            .map(|line| Gate::new(line))
            .map(|x| (x.output.clone(), x))
            .collect();
        Self {
            values,
            connections,
        }
    }

    fn evaluate(&mut self) {
        for _ in 0..self.connections.len() {
            for mut gate in self.connections.values_mut() {
                match gate.evaluate(&self.values) {
                    Some(result) => {
                        self.values.insert(gate.output.name.clone(), result);
                    }
                    None => {}
                }
            }
        }
    }

    fn extract_z(&self) -> u64 {
        let mut output = String::new();
        for a in 0..10 {
            for b in 0..10 {
                let key = format!("z{a}{b}");
                let next = self.values.get(&key);
                match next {
                    None => {
                        let binary_string = output.chars().rev().collect::<String>();
                        return u64::from_str_radix(binary_string.as_str(), 2).unwrap();
                    }
                    Some(value) => {
                        output += value.to_string().as_str();
                    }
                }
            }
        }
        0
    }
}

fn dump_graph(wires: &Wires) {
    let mut writer = BufWriter::new(File::create("graph.dot").unwrap());
    writer.write_all(b"digraph {{\n").unwrap();
    for gate in wires.connections.values() {
        let color = match gate.operation {
            Operation::OR => "blue",
            Operation::AND => "yellow",
            Operation::XOR => "red",
        };
        writer
            .write_all(
                format!("{} [style=filled,fillcolor={}];\n", gate.output.name, color).as_bytes(),
            )
            .unwrap();
    }
    for g in wires.connections.values() {
        for input in &g.inputs {
            writer
                .write_all(format!("{} -> {};\n", input.name, g.output.name).as_bytes())
                .unwrap();
        }
    }
    writer.write_all(b"}}").unwrap();
}

fn part2(wires: &Wires) -> String {
    for gate in wires.connections.values() {
        if gate.output.is_z() && gate.output.index < 45 {
            if gate.operation != Operation::XOR {
                println!(
                    "{:?} = {:?} from {:?} but should be XOR",
                    gate.output, gate.operation, gate.inputs
                );
            } else {
                for input in &gate.inputs {
                    let input_gate = wires.connections.get(&input);
                    match input_gate {
                        Some(g) => {
                            if g.operation == Operation::AND {
                                println!("{:?} is AND and should be XOR or OR", g)
                            } else if g.operation == Operation::XOR {
                                //zij = xij^yij
                                if !g.inputs.iter().all(|x| x.index == gate.output.index) {
                                    println!(
                                        "{:?} should be x{}^y{}",
                                        g, gate.output.index, gate.output.index
                                    )
                                }
                            } else {
                                if !g.inputs.iter().all(|x| {
                                    wires.connections.get(&x).map(|x| &x.operation)
                                        == Some(&Operation::AND)
                                }) {
                                    println!("{:?} should be AND", g)
                                }
                            }
                        }
                        None => {
                            println!("{:?} is direct input and should be XOR or OR", input);
                        }
                    }
                }
            }
        }
    }
    dump_graph(&wires);
    // manually inspect indicated nodes to figure out what swapped :P
    let swaps = "mwk,z10,hsw,jmh,qgd,z18,gqp,z33";
    swaps.split(",").sorted().join(",")
}
fn part1(wires: &mut Wires) -> u64 {
    wires.evaluate();
    wires.extract_z()
}
pub(crate) fn solve() {
    let content = fs::read_to_string("24.txt").unwrap();
    let mut wires = Wires::new(&content);
    println!("{}", part1(&mut wires));
    println!("{}", part2(&wires));
}
