use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| match ch {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Unexpected character!"),
                })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
fn toboggan_at_fixed_slope(input: &Vec<Vec<u8>>) -> usize {
    toboggan_at_slope(input, (3, 1))
}

fn toboggan_at_slope(input: &Vec<Vec<u8>>, slope: (usize, usize)) -> usize {
    let (slope_x, slope_y) = slope;
    let mut xpos = 0;
    let mut tree_count = 0;
    for row in input.iter().step_by(slope_y) {
        if *row.get(xpos % (row.len())).unwrap() == 1 {
            tree_count += 1;
        }
        xpos += slope_x;
    }
    tree_count
}

#[aoc(day3, part2)]
fn toboggan_at_other_slopes(input: &Vec<Vec<u8>>) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|s| toboggan_at_slope(input, *s))
        .fold(1, |memo, x| memo * x)
}

#[cfg(test)]
#[test]
fn test_toboggan() {
    let test_input = "..##.......\n\
                      #...#...#..\n\
                      .#....#..#.\n\
                      ..#.#...#.#\n\
                      .#...##..#.\n\
                      ..#.##.....\n\
                      .#.#.#....#\n\
                      .#........#\n\
                      #.##...#...\n\
                      #...##....#\n\
                      .#..#...#.#";
    assert_eq!(7, toboggan_at_fixed_slope(&parse_input_day3(test_input)));
}

#[test]
fn test_toboggan_multi_slope() {
    let test_input = "..##.......\n\
                      #...#...#..\n\
                      .#....#..#.\n\
                      ..#.#...#.#\n\
                      .#...##..#.\n\
                      ..#.##.....\n\
                      .#.#.#....#\n\
                      .#........#\n\
                      #.##...#...\n\
                      #...##....#\n\
                      .#..#...#.#";
    assert_eq!(336, toboggan_at_other_slopes(&parse_input_day3(test_input)));
}
