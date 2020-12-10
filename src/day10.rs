use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> Vec<usize> {
    input.lines().map(|line| str::parse(line).unwrap()).collect()
}

#[aoc(day10, part1)]
fn joltage_distribution(input: &Vec<usize>) -> usize {
    let mut jolt_order = input.clone();
    jolt_order.insert(0,0);
    jolt_order.sort();
    let mut distribution = HashMap::<usize, Vec<usize>>::new();
    let mut order = jolt_order.iter().peekable();

    while let Some(val) = order.next() {
        if let Some(next) = order.peek() {
            match distribution.get_mut(&(*next - val)) {
                Some(d_vec) => d_vec.push(*next.clone()),
                None => {distribution.insert(*next - val, vec![*next.clone()]); ()}
            }
        } else {
            let d_vec = distribution.get_mut(&3).expect("failed to find key");
            d_vec.push(val+3);
        }
    }
    println!("{:?}", jolt_order);
    distribution[&1].iter().count() * distribution[&3].iter().count()
}

/* #[aoc(day10, part2)]
 fn adapter_combinations(input: &Vec<usize>) -> u64 {
    let mut jolt_order = input.clone();
    jolt_order.insert(0,0);
    let mut mults: Vec<usize> = Vec::new();
    let mut jolt_iter = jolt_order.iter().peekable();
    3
} */

#[cfg(test)]

#[test]
fn test_part_1() {
    let test_str = "\
16
10
15
5
1
11
7
19
6
12
4";

    assert_eq!(35, joltage_distribution(&parse_input_day10(test_str)));
}

#[test]
fn test_competing_iter() {
    let test_str = "\
16
10
15
5
1
11
7
19
6
12
4";
    let nums = parse_input_day10(test_str);
    let mut nums_iter = nums.iter();
    assert_eq!(Some(&16), nums_iter.next());
}
