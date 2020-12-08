use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

struct Program {
    instructions: Vec<Instruction>,
    accumulator: isize,
    fp: isize,
    clean_exit: bool,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        Program {
            instructions: instructions,
            accumulator: 0,
            fp: 0,
            clean_exit: true,
        }
    }
    fn run_until_cycle(&mut self) {
        loop {
            let mut inst = self.instructions.get_mut(self.fp as usize);
            match inst {
                Some(inst) => {
                    inst.visit_count += 1;
                    if inst.visit_count == 2 {
                        self.clean_exit = false;
                        return;
                    }
                    match &inst.opcode[..] {
                        "acc" => {
                            self.accumulator += inst.position;
                            self.fp += 1;
                        }
                        "jmp" => self.fp += inst.position,
                        "nop" => self.fp += 1,
                        _ => panic!("Unexpected instruction!"),
                    }
                }
                None => return,
            }
        }
    }
}

#[derive(Clone)]
struct Instruction {
    opcode: String,
    position: isize,
    visit_count: u8,
}

#[aoc_generator(day8)]
fn parse_input_day8(input: &str) -> Vec<Instruction> {
    let instruction_re =
        Regex::new("^(?P<opcode>\\w{3}) (?P<pos>[+-]\\d+)").expect("Couldn't create regex!");
    input
        .lines()
        .map(|line| {
            let captures = instruction_re.captures(line).expect("Didn't match regex!");
            Instruction {
                opcode: String::from(captures.name("opcode").unwrap().as_str()),
                position: str::parse(captures.name("pos").unwrap().as_str()).unwrap(),
                visit_count: 0,
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn read_accumulator_at_cycle(input: &Vec<Instruction>) -> isize {
    let insts = input.clone();
    let mut pg = Program::new(insts);
    pg.run_until_cycle();
    pg.accumulator
}

#[aoc(day8, part2)]
fn correct_broken_op(input: &Vec<Instruction>) -> isize {
    let mut pg: Program;
    let mut result: isize = 0;
    for (pos, inst) in input.iter().enumerate() {
        if &inst.opcode[..] == "nop" {
            let mut insts = input.clone();
            insts.get_mut(pos).unwrap().opcode = String::from("jmp");
            pg = Program::new(insts);
            pg.run_until_cycle();
            if pg.clean_exit {
                result = pg.accumulator;
                break;
            }
        }
        if &inst.opcode[..] == "jmp" {
            let mut insts = input.clone();
            insts.get_mut(pos).unwrap().opcode = String::from("nop");
            pg = Program::new(insts);
            pg.run_until_cycle();
            if pg.clean_exit {
                result = pg.accumulator;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
#[test]
fn test_part_1() {
    let test_str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(5, read_accumulator_at_cycle(&parse_input_day8(test_str)));
}

#[test]
fn test_part_2() {
    let test_str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    assert_eq!(8, correct_broken_op(&parse_input_day8(test_str)));
}
