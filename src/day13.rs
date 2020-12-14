use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Bus {
    interval: u128,
    last_pickup: u128,
}

#[derive(Clone)]
struct Timetable {
    depart: usize,
    buses: Vec<Option<Bus>>,
}

impl Bus {
    fn new(interval: u128) -> Bus {
        Bus {
            interval,
            last_pickup: 0,
        }
    }
}
impl Iterator for Bus {
    type Item = u128;
    fn next(&mut self) -> Option<u128> {
        self.last_pickup += self.interval as u128;
        Some(self.last_pickup)
    }
}

#[aoc_generator(day13)]
fn parse_input_day13(input: &str) -> Timetable {
    let lines: Vec<&str> = input.lines().collect();
    let approximate_departure = str::parse(lines.get(0).unwrap());
    let buses = lines.get(1).unwrap().split(",");
    Timetable {
        depart: approximate_departure.unwrap(),
        buses: buses
            .map(|b| {
                match b {
                    "x" => {
                        // Skip out-of-service buses
                        None
                    }
                    _ => Some(Bus::new(str::parse(b).unwrap())),
                }
            })
            .collect(),
    }
}

#[aoc(day13, part1)]
fn find_next_bus(root_input: &Timetable) -> u128 {
    let input = root_input.clone();
    let next_bus = input
        .buses
        .iter()
        .filter_map(|b| match b.as_ref() {
            None => None,
            Some(bus) => Some(Bus {
                interval: bus.interval,
                last_pickup: bus
                    .clone()
                    .take_while(|t| *t < input.depart as u128 + bus.interval)
                    .last()
                    .unwrap(),
            }),
        })
        .min_by(|bus1, bus2| bus1.last_pickup.cmp(&bus2.last_pickup))
        .unwrap();

    println!("{:?}", next_bus);
    next_bus.interval as u128 * (next_bus.last_pickup - input.depart as u128)
}

#[aoc(day13, part2)]
fn consecutive_buses(root_input: &Timetable) -> u128 {
    let input = root_input.clone();
    let mut result: u128 = 0;
    let mut sorted_buses: Vec<(usize, &Bus)> = input
        .buses
        .iter()
        .enumerate()
        .filter_map(|(idx, bus)| {
            match bus {
                Some(b) => Some((idx, b)),
                None => None
            }
        }).collect();
        sorted_buses.sort_by(|(_, bus1), (_, bus2)| bus2.interval.cmp(&bus1.interval));
        println!("{:?}", sorted_buses);
        let mut increment = 1;
        sorted_buses.iter().for_each(|(idx, bus)| {
            while (result + *idx as u128) % bus.interval != 0 {
                result += increment
            }
            increment *= bus.interval;
        });
    result
}

#[cfg(test)]
#[test]
fn test_part_1() {
    let test_str = "939\n7,13,x,x,59,x,31,19";
    assert_eq!(295, find_next_bus(&parse_input_day13(test_str)));
}

#[test]
fn test_part_2() {
    let test_str = "939\n7,13,x,x,59,x,31,19";
    let test_str2 = "939\n17,x,13,19";
    let test_str3 = "939\n67,7,59,61";
    let test_str4_2 = "939\n67,x,7,59,61";
    let test_str4 = "939\n67,7,x,59,61";
    let test_str5 = "939\n1789,37,47,1889";

    assert_eq!(1068781, consecutive_buses(&parse_input_day13(test_str)));
    assert_eq!(3417, consecutive_buses(&parse_input_day13(test_str2)));
    assert_eq!(754018, consecutive_buses(&parse_input_day13(test_str3)));
    assert_eq!(779210, consecutive_buses(&parse_input_day13(test_str4_2)));
    assert_eq!(1261476, consecutive_buses(&parse_input_day13(test_str4)));
    assert_eq!(1202161486, consecutive_buses(&parse_input_day13(test_str5)));
}
