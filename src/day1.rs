use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<usize> {
    input.lines().map(|v| usize::from_str_radix(v, 10).unwrap()).collect()
}

#[aoc(day1, part1)]
fn find_2020_entries(input: &Vec<usize>) -> usize {
    let pair = input.iter().enumerate().find_map(|(idx, &item)| {
        match input.iter().enumerate().find(|(add_idx, &addend)| {
            add_idx != &idx && (item + addend == 2020)
        }) {
            Some((idx, addend)) => Some(item * addend),
            None => None
        }
    });
    pair.unwrap_or(1)
}


#[cfg(test)]

#[test]
fn test_find_2020_entries() {
    let test_vec = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(514579, find_2020_entries(&test_vec))
}

#[test]
fn test_find_2020_entries_not_self() {
    let test_vec = vec![1010];

    assert_eq!(1, find_2020_entries(&test_vec))
}

#[test]
fn test_find_2020_entries_double_self() {
    let test_vec = vec![1010, 1010];

    assert_eq!(1020100, find_2020_entries(&test_vec))
}
