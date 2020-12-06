use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|group| String::from(group))
        .collect()
}

#[aoc(day6, part1)]
fn sum_group_questions(input: &Vec<String>) -> usize {
    let answers: Vec<HashSet<char>> = input
        .iter()
        .map(|group| group.chars().filter(|x| *x != '\n').collect())
        .collect();
    answers.iter().map(|group| group.len()).sum()
}

#[aoc(day6, part2)]
fn sum_group_common_questions(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|group| {
            let mut first_run = true;
            group
                .lines()
                .map(|answers| answers.chars().collect::<HashSet<char>>())
                .fold(HashSet::new(), |memo, ans| {
                    if memo.is_empty() && first_run {
                        first_run = false;
                        ans.clone()
                    } else {
                        let intersect = memo.intersection(&ans).cloned().collect();
                        intersect
                    }
                })
                .len()
        })
        .sum()
}

#[cfg(test)]
#[test]
fn test_common_answers() {
    let test_input = "abc

a
b
c

ab
ac

a
a
a
a

b
";
    assert_eq!(3, sum_group_common_questions(&vec![String::from("abc")]));
    assert_eq!(
        3,
        sum_group_common_questions(&vec![String::from("abc"), String::from("a\nb\nc")])
    );
    assert_eq!(6, sum_group_common_questions(&parse_input_day6(test_input)));
}
