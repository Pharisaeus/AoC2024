use itertools::Itertools;
use std::fs;

#[derive(Debug)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl Opcode {
    fn new(opcode: &str) -> Self {
        match opcode {
            "0" => Opcode::ADV,
            "1" => Opcode::BXL,
            "2" => Opcode::BST,
            "3" => Opcode::JNZ,
            "4" => Opcode::BXC,
            "5" => Opcode::OUT,
            "6" => Opcode::BDV,
            "7" => Opcode::CDV,
            x => panic!("Bad opcode: {}", x),
        }
    }
}
#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    operand: i64,
}

impl Instruction {
    fn new(opcode: &str, operand: &str) -> Self {
        Self {
            opcode: Opcode::new(opcode),
            operand: operand.to_string().parse().unwrap(),
        }
    }
}
struct Machine {
    ip: usize,
    a: isize,
    b: isize,
    c: isize,
    instructions: Vec<Instruction>,
    output: Vec<isize>,
}

impl Machine {
    fn new(content: &str) -> Self {
        let (regs, program) = content.split("\n\n").collect_tuple().unwrap();
        let (a, b, c) = regs
            .lines()
            .map(|line| line[12..].parse().unwrap())
            .collect_tuple()
            .unwrap();
        let instructions = program[9..]
            .split(",")
            .collect_vec()
            .chunks(2)
            .map(|instruction| Instruction::new(instruction[0], instruction[1]))
            .collect();
        Self {
            ip: 0,
            a,
            b,
            c,
            instructions,
            output: Vec::new(),
        }
    }

    fn run_program(&mut self) {
        while self.ip < self.instructions.len() {
            self.single_step()
        }
    }

    fn single_step(&mut self) {
        let instruction = &self.instructions[self.ip];
        let mut step = true;
        match instruction.opcode {
            Opcode::ADV => {
                self.a = self.division(instruction.operand);
            }
            Opcode::BXL => {
                self.b = self.b ^ instruction.operand as isize;
            }
            Opcode::BST => {
                self.b = self.combo_operand(instruction.operand) % 8;
            }
            Opcode::JNZ => {
                if self.a != 0 {
                    self.ip = instruction.operand as usize;
                    step = false;
                }
            }
            Opcode::BXC => {
                self.b = self.b ^ self.c;
            }
            Opcode::OUT => {
                self.output
                    .push(self.combo_operand(instruction.operand) % 8);
            }
            Opcode::BDV => {
                self.b = self.division(instruction.operand);
            }
            Opcode::CDV => {
                self.c = self.division(instruction.operand);
            }
        }
        if step {
            self.ip += 1;
        }
    }

    fn division(&self, operand: i64) -> isize {
        self.a >> self.combo_operand(operand)
    }

    fn combo_operand(&self, operand: i64) -> isize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!(),
            _ => panic!(),
        }
    }

    fn reset(&mut self) {
        self.output.clear();
        self.ip = 0;
    }
}

fn part2(machine: &mut Machine, target: &Vec<isize>) -> isize {
    // Code is doing:
    // while a!= 0
    // 	b = (a%8) xor 1
    // 	c = a >> b
    // 	out (b xor 5 xor c) % 8
    // 	a = a >> 3
    // we can match suffix, shift by 3 and try to match a longer one
    // every time we do << 3 we get another digit in the output
    let mut current_a = 0;
    for match_len in 1..=target.len() {
        let expected_suffix = &target[target.len() - match_len..];
        current_a <<= 3;
        for low_bits in 0..64 {
            machine.reset();
            machine.a = current_a + low_bits;
            machine.run_program();
            if machine.output.eq(expected_suffix) {
                current_a = current_a + low_bits;
                break;
            }
        }
    }
    current_a
}
fn part1(machine: &mut Machine) -> String {
    machine.run_program();
    machine.output.iter().join(",")
}
pub(crate) fn solve() {
    let content = fs::read_to_string("17.txt").unwrap();
    let (_, program) = content.split("\n\n").collect_tuple().unwrap();
    let target: Vec<isize> = program[9..]
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect_vec();
    let mut machine = Machine::new(&content);
    println!("{}", part1(&mut machine));
    println!("{}", part2(&mut machine, &target));
}
