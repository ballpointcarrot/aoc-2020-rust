use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
    Left,
    Right,
    Forward,
}

struct Command {
    direction: Direction,
    measure: usize,
}

#[aoc_generator(day12)]
fn parse_input_day12(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| Command {
            direction: match l.chars().next() {
                Some('F') => Direction::Forward,
                Some('N') => Direction::North,
                Some('S') => Direction::South,
                Some('E') => Direction::East,
                Some('W') => Direction::West,
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => panic!("Invalid direction!"),
            },
            measure: str::parse(&l[1..]).expect("Unable to parse measure"),
        })
        .collect()
}

fn manhattan_distance(point_1: &(isize, isize), point_2: &(isize, isize)) -> usize {
    ((point_1.0 - point_2.0).abs() + (point_1.1 - point_2.1).abs()) as usize
}

fn move_ship(position: &mut (isize, isize), direction: &Direction, distance: usize) {
    match *direction {
        Direction::North => {
            position.1 += distance as isize;
        }
        Direction::South => {
            position.1 -= distance as isize;
        }
        Direction::East => {
            position.0 -= distance as isize;
        }
        Direction::West => {
            position.0 += distance as isize;
        }
        _ => panic!("cannot move in specified Direction {:?}", direction),
    }
}

fn angle_lookup(direction: &Direction) -> usize {
    match direction {
        Direction::North => 270,
        Direction::South => 90,
        Direction::East => 0,
        Direction::West => 180,
        _ => panic!("not a cardinal direction"),
    }
}

fn direction_lookup(angle: usize) -> Direction {
    match angle {
        0 => Direction::East,
        90 => Direction::South,
        180 => Direction::West,
        270 => Direction::North,
        _ => panic!("invalid angle"),
    }
}

fn turn_ship(orientation: &mut Direction, command: &Command) {
    let mut degree: isize = match command.direction {
        Direction::Left => -1 * command.measure as isize,
        Direction::Right => command.measure as isize,
        _ => panic!(
            "cannot turn with specified Direction {:?}",
            command.direction
        ),
    };

    let mut current_angle = angle_lookup(orientation) as isize;
    while degree != 0 {
        if degree < 0 {
            current_angle -= 90;
            degree += 90;
        } else {
            current_angle += 90;
            degree -= 90;
        }
    }
    let final_angle: usize = match current_angle.cmp(&0) {
        Ordering::Less => (current_angle % 360) + 360,
        _ => current_angle % 360,
    } as usize;
    *orientation = direction_lookup(final_angle);
}

#[aoc(day12, part1)]
fn get_travel_distance(input: &Vec<Command>) -> usize {
    let origin = (0, 0);
    let mut position = (0, 0);
    let mut orientation = Direction::East;

    input.iter().for_each(|cmd| match cmd.direction {
        Direction::North | Direction::South | Direction::East | Direction::West => {
            move_ship(&mut position, &cmd.direction, cmd.measure);
        }
        Direction::Forward => {
            move_ship(&mut position, &orientation, cmd.measure);
        }
        Direction::Left | Direction::Right => {
            turn_ship(&mut orientation, &cmd);
        }
    });

    manhattan_distance(&origin, &position)
}

fn move_waypoint(waypoint: &mut (isize, isize), cmd: &Command) {
    match cmd.direction {
        Direction::East => waypoint.0 += cmd.measure as isize,
        Direction::West => waypoint.0 -= cmd.measure as isize,
        Direction::North => waypoint.1 += cmd.measure as isize,
        Direction::South => waypoint.1 -= cmd.measure as isize,
        _ => panic!("Not a move Direction"),
    }
}

fn rotate_waypoint(waypoint: &mut (isize, isize), cmd: &Command) {
    let angle = match cmd.direction {
        Direction::Left => ((-1 * cmd.measure as isize) + 360) as usize,
        Direction::Right => cmd.measure,
        _ => panic!("Not a rotational direction"),
    };

    match angle {
        0 => (),
        270 => {
            let new_x = -1 * waypoint.1;
            let new_y = waypoint.0;
            waypoint.0 = new_x;
            waypoint.1 = new_y;
        }
        180 => {
            waypoint.0 = -1 * waypoint.0;
            waypoint.1 = -1 * waypoint.1;
        }
        90 => {
            let new_x = waypoint.1;
            let new_y = -1 * waypoint.0;
            waypoint.0 = new_x;
            waypoint.1 = new_y;
        }
        _ => panic!("invalid angle"),
    }
}

fn move_ship_toward_waypoint(
    position: &mut (isize, isize),
    waypoint: &(isize, isize),
    times: usize,
) {
    for _ in 0..times {
        position.0 += waypoint.0;
        position.1 += waypoint.1;
    }
}

#[aoc(day12, part2)]
fn get_travel_waypoint_distance(input: &Vec<Command>) -> usize {
    let origin = (0, 0);
    let mut waypoint = (10, 1);
    let mut position = (0, 0);

    input.iter().for_each(|cmd| {
        match cmd.direction {
            Direction::North | Direction::South | Direction::East | Direction::West => {
                move_waypoint(&mut waypoint, cmd);
            }
            Direction::Forward => {
                move_ship_toward_waypoint(&mut position, &waypoint, cmd.measure);
            }
            Direction::Left | Direction::Right => rotate_waypoint(&mut waypoint, &cmd),
        }
    });

    manhattan_distance(&origin, &position)
}

#[cfg(test)]
#[test]
fn test_manhattan_distance() {
    assert_eq!(0, manhattan_distance(&(0, 0), &(0, 0)));
    assert_eq!(2, manhattan_distance(&(0, 0), &(1, 1)));
}

#[test]
fn test_part1_input() {
    let input_str = "F10\nN3\nF7\nR90\nF11";
    assert_eq!(25, get_travel_distance(&parse_input_day12(input_str)));
}

#[test]
fn test_part2_input() {
    let input_str = "F10\nN3\nF7\nR90\nF11";
    assert_eq!(
        286,
        get_travel_waypoint_distance(&parse_input_day12(input_str))
    );
}
