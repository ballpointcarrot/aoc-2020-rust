use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use std::iter::repeat;

struct MaskInstruction {
    mask: String,
}

struct AssignInstruction {
    address: usize,
    value: isize,
}

enum Instruction {
    Mask(MaskInstruction),
    Assign(AssignInstruction),
}

#[aoc_generator(day14)]
fn parse_input_day14(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|s| match s.starts_with("mask") {
            true => Instruction::Mask(MaskInstruction {
                mask: String::from(s.split(" ").last().unwrap()),
            }),
            false => {
                let re = Regex::new("\\[(\\d+)\\] = (-?\\d+)").expect("Unable to build regex");
                let captures = re.captures(s).unwrap();
                Instruction::Assign(AssignInstruction {
                    address: str::parse(captures.get(1).unwrap().as_str()).unwrap(),
                    value: str::parse(captures.get(2).unwrap().as_str()).unwrap(),
                })
            }
        })
        .collect()
}

fn into_binary_string(value: &isize, s: &mut Vec<char>) {
    let mut val = value.clone();
    let mut idx = 0;
    while val != 0 {
        match val & 1 {
            1 => s.insert(idx, '1'),
            0 => s.insert(idx, '0'),
            _ => panic!("unreachable"),
        }
        val = val >> 1;
        idx += 1;
    }
}

fn masked(value: &isize, mask: &mut String) -> usize {
    let mut val_bin = repeat('0').take(36).collect::<Vec<_>>();
    into_binary_string(value, &mut val_bin);
    for (idx, ch) in mask.chars().rev().enumerate() {
        match ch {
            '0' | '1' => val_bin[idx] = ch,
            _ => (),
        }
    }
    val_bin.reverse();
    usize::from_str_radix(val_bin.iter().collect::<String>().as_str(), 2).expect("invalid number!")
}

#[aoc(day14, part1)]
fn do_thing(input: &Vec<Instruction>) -> usize {
    let mut memory = HashMap::new();
    let mut current_mask = String::from("");

    input.iter().for_each(|inst| match inst {
        Instruction::Mask(msk) => {
            current_mask = msk.mask.clone();
        }
        Instruction::Assign(assign) => {
            memory.insert(&assign.address, masked(&assign.value, &mut current_mask));
        }
    });
    memory.values().sum()
}

#[cfg(test)]
#[test]
fn test_mask() {
    assert_eq!(7, masked(&0, &mut String::from("111")));
}

#[test]
fn test_input_part1() {
    let test_input =
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
    assert_eq!(165, do_thing(&parse_input_day14(test_input)));
}
