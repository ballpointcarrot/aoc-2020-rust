use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[aoc_generator(day9)]
fn parse_input_day9(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| str::parse(line).unwrap())
        .collect()
}

fn process_xmas(input: &Vec<isize>, preamble_count: usize) -> &isize {
    let mut buffer: Vec<isize> = input.iter().copied().take(preamble_count).collect();
    input
        .iter()
        .skip(preamble_count)
        .find(|num| {
            match buffer.iter().find(|buf_num| {
                buffer
                    .iter()
                    .any(|buf_addend| *buf_addend + *buf_num == **num)
            }) {
                Some(_n) => {
                    buffer.push(**num);
                    buffer.remove(0);
                    false
                }
                None => true,
            }
        })
        .unwrap()
}

#[aoc(day9, part1)]
fn find_target_value(input: &Vec<isize>) -> isize {
    let preamble_count = 25;
    *process_xmas(input, preamble_count)
}

fn process_weak_point(input: &Vec<isize>, preamble_count: usize) -> isize {
    let target_value = *process_xmas(input, preamble_count);
    let mut span: &[isize];
    for (idx, _) in input.iter().enumerate() {
        let mut slider = idx.clone();
        let mut over = false;
        while !over {
            span = &input[idx..slider];
            match target_value.cmp(&span.iter().sum::<isize>()) {
                Ordering::Greater => {
                    slider += 1;
                }
                Ordering::Equal => {
                    return *span.iter().min().unwrap() + *span.iter().max().unwrap();
                }
                Ordering::Less => {
                    over = true;
                }
            }
        }
    }
    -1
}

#[aoc(day9, part2)]
fn hit_weak_point_for_massive_damage(input: &Vec<isize>) -> isize {
    let preamble_count = 25;
    process_weak_point(input, preamble_count)
}

#[cfg(test)]
#[test]
fn test_input_1() {
    let test_str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    assert_eq!(&127, process_xmas(&parse_input_day9(test_str), 5));
}

#[test]
fn test_input_part_2() {
    let test_str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    assert_eq!(
        62,
        process_weak_point(&parse_input_day9(test_str), 5)
    );
}
