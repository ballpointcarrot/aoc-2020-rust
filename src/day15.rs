use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Game {
    plays: HashMap<isize, Option<usize>>,
    numbers: Vec<isize>,
    last: isize,
}

impl Game {
    fn new(numbers: Vec<isize>) -> Game {
        let mut plays = HashMap::new();
        numbers.iter().for_each(|num| {
            plays.insert(*num, None);
        });
        Game {
            plays,
            numbers: numbers.clone(),
            last: numbers.iter().last().unwrap().clone(),
        }
    }
}

impl Iterator for Game {
    type Item = Game;

    fn next(&mut self) -> Option<Game> {
        let current_length = self.numbers.len();
        let next_num: isize = match self.plays.get(&self.last) {
            Some(pos) => match pos {
                Some(n) => current_length as isize - *n as isize,
                None => 0,
            },
            None => 0,
        };
        Some(Game {
            plays: {
                self.plays.insert(next_num, Some(current_length));
                self.plays.clone()
            },
            numbers: {
                self.numbers.push(next_num);
                self.numbers.clone()
            },
            last: next_num,
        })
    }
}

#[aoc_generator(day15)]
fn parse_input_day15(input: &str) -> Vec<isize> {
    input.split(",").map(|v| str::parse(v).unwrap()).collect()
}

#[aoc(day15, part1)]
fn play_game(input: &Vec<isize>) -> isize {
    let game = Game::new(input.clone());
    let list = game.take(2020).last().unwrap();
    println!("{:?}", list.numbers);
    println!("{:?}", list.plays);
    list.last
}

#[cfg(test)]
#[test]
fn test_inputs_part1() {
    assert_eq!(436, play_game(&vec![0, 3, 6]));
}
