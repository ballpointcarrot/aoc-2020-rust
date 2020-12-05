use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Vec<String> {
    input.lines().map(|v| String::from(v)).collect()
}

#[aoc(day5, part1)]
fn highest_pass(passes: &Vec<String>) -> usize {
    passes.iter().map(|p| pass_id(p)).max().unwrap()
}

fn pass_id(pass: &String) -> usize {
    let position = pass.chars().fold((0, 0), |memo, ch| {
        match ch {
            'F' => (memo.0 << 1, memo.1),
            'B' => ((memo.0 << 1) | 1, memo.1),
            'L' => (memo.0, memo.1 << 1),
            'R' => (memo.0, (memo.1 << 1) | 1),
            _ => panic!("Invalid character")
        }
    });
    position.0 * 8 + position.1
}

#[aoc(day5, part2)]
fn find_my_seat(passes: &Vec<String>) -> usize {
    let mut all_pass_ids: Vec<usize> = passes.iter().map(|p| pass_id(p)).collect();
    all_pass_ids.sort();
    1 + all_pass_ids.iter().fold(0, |memo, n| {
        if memo == 0 || memo + 1 == *n {
            *n
        } else {
            memo
        }
    })
}

#[cfg(test)]

#[test]
fn test_read_pass() {
    let pass = String::from("FBFBBFFRLR");
    assert_eq!(357, pass_id(&pass));
}

#[test]
fn sanity_test() {
    assert_eq!(4, 2 << 1);
    assert_eq!(13, (6 << 1) | 1);

}
