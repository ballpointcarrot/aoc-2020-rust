use aoc_runner_derive::{aoc, aoc_generator};

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

struct PasswordChallenge {
    password: String,
    min: usize,
    max: usize,
    search_char: char,
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<PasswordChallenge> {
    input.lines().map(|v| parse_line(v)).collect()
}

fn parse_line(line: &str) -> PasswordChallenge {
    let pattern = regex!(r"^(\d+)-(\d+)\s(\w):\s(\w+)$");
    let captures = pattern.captures(line).unwrap();

    PasswordChallenge {
        min: str::parse(captures.get(1).unwrap().as_str()).unwrap(),
        max: str::parse(captures.get(2).unwrap().as_str()).unwrap(),
        search_char: (captures.get(3).unwrap().as_str()).chars().next().unwrap(),
        password: String::from(captures.get(4).unwrap().as_str()),
    }
}

#[aoc(day2, part1)]
fn valid_password_count(input: &Vec<PasswordChallenge>) -> usize {
    input
        .iter()
        .filter(|challenge| is_valid_password_by_char_count(challenge))
        .count()
}

fn is_valid_password_by_char_count(challenge: &PasswordChallenge) -> bool {
    let pw_chars_count = challenge
        .password
        .chars()
        .filter(|ch| ch == &challenge.search_char)
        .count();

    challenge.min <= pw_chars_count && challenge.max >= pw_chars_count
}

#[aoc(day2, part2)]
fn valid_password_positions(input: &Vec<PasswordChallenge>) -> usize {
    input
        .iter()
        .filter(|challenge| is_valid_password_by_pos(challenge))
        .count()
}

fn is_valid_password_by_pos(challenge: &PasswordChallenge) -> bool {
    challenge
        .password
        .char_indices()
        .fold(false, |memo, (idx, ch)| {
            if idx == challenge.min - 1 || idx == challenge.max - 1 {
                memo ^ (ch == challenge.search_char)
            } else {
                memo
            }
        })
}
