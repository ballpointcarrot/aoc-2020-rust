use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, PartialEq)]
struct SeatLayout {
    layout: Vec<Vec<Option<bool>>>
}

impl SeatLayout {
    fn get_bounds(&self, ypos: usize, xpos: usize) -> Vec<bool> {
        self.layout.iter().enumerate().filter(|(y_pos, _)| {
            let distance: isize = (ypos as isize) - (*y_pos as isize);
            distance == 1 || distance == 0 || distance == -1
        }).map(|(y_pos, row)| {
            row.iter().enumerate().filter(|(x_pos, _)| {
                let distance: isize = (xpos as isize) - (*x_pos as isize);
                distance == 1 || distance == 0 || distance == -1
            }).filter_map(|(x_pos, val)| {
                if x_pos == xpos && y_pos == ypos {
                    None
                } else {
                    *val
                }
            }).collect::<Vec<bool>>()
        }).flatten().collect()
    }

    fn gen(&self) -> SeatLayout {
        let mut next_gen = self.clone();
        next_gen.layout = next_gen.layout.iter().enumerate().map(|(ypos, y)| {
            y.iter().enumerate().map(|(xpos, _)| {
                if let Some(seat) = next_gen.layout[ypos][xpos] {
                    let bounds = self.get_bounds(ypos, xpos);
                    match bounds.iter().filter(|x| **x).count() {
                        0 => Some(true),
                        4..=9 => Some(false),
                        _ => Some(seat)
                    }
                } else {
                    None
                }
            }).collect()
        }).collect();
        next_gen
    }
}

#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> SeatLayout {
    SeatLayout {
        layout: input.lines().map(|l| {
            l.chars().map(|ch| {
                match ch {
                    'L' => Some(false),
                    '#' => Some(true),
                    '.' => None,
                    _ => panic!("Invalid input")
                }
            }).collect()
        }).collect()
    }
}

#[aoc(day11, part1)]
fn count_occupied_seats(input: &SeatLayout) -> usize {
    let mut this_gen = input.clone();
    let mut next_gen = this_gen.gen();
    while this_gen !=next_gen {
        this_gen = next_gen;
        next_gen = this_gen.gen()
    }
    next_gen.layout.iter().filter_map(|row| {
        match row.iter().filter(|col| {
            **col == Some(true)
        }).count() {
            0 => None,
            n => Some(n)
        }
    }).sum()
}

#[cfg(test)]
#[test]
fn test_gen() {
let gen0 = parse_input_day11("LLL\nLLL");
let gen1 = parse_input_day11("###\n###");
assert_eq!(gen1.layout, gen0.gen().layout);
}

#[test]
fn test_inputs_part1() {
    let in_str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    let gen1 = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

let gen2 = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

    assert_eq!(parse_input_day11(gen1).layout, parse_input_day11(in_str).gen().layout);
    assert_eq!(parse_input_day11(gen2).layout, parse_input_day11(in_str).gen().gen().layout);

}
