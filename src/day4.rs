use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Height {
    measure: usize,
    unit: String,
}

impl Height {
    fn parse(hgt_str: &str) -> Option<Height> {
        let re = Regex::new("(\\d+)(in|cm)").expect("Unable to create Regex");
        match re.captures(hgt_str) {
            None => None,
            Some(captures) => {
                let h = Height {
                    measure: str::parse(captures.get(1).unwrap().as_str())
                        .expect("Unable to parse number"),
                    unit: String::from(captures.get(2).unwrap().as_str()),
                };
                Some(h)
            }
        }
    }
    fn is_valid(&self) -> bool {
        match self.unit.as_str() {
            "cm" => self.measure >= 150 && self.measure <= 193,
            "in" => self.measure >= 59 && self.measure <= 76,
            _ => panic!("Not a valid unit"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<Height>,
    hgt_str: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hgt_str: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn has_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt_str.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        self.valid_byr()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_ecl()
            && self.valid_pid()
    }

    fn valid_byr(&self) -> bool {
        match self.byr {
            None => false,
            Some(n) => n >= 1920 && n <= 2002,
        }
    }
    fn valid_iyr(&self) -> bool {
        match self.iyr {
            None => false,
            Some(n) => n >= 2010 && n <= 2020,
        }
    }
    fn valid_eyr(&self) -> bool {
        match self.eyr {
            None => false,
            Some(n) => n >= 2020 && n <= 2030,
        }
    }
    fn valid_hgt(&self) -> bool {
        match &self.hgt {
            None => false,
            Some(h) => h.is_valid(),
        }
    }
    fn valid_hcl(&self) -> bool {
        let re = Regex::new("^#[0-9a-f]{6}$").expect("Failed to make regex");
        match &self.hcl {
            None => false,
            Some(hair) => re.is_match(hair.as_str()),
        }
    }
    fn valid_ecl(&self) -> bool {
        let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match &self.ecl {
            None => false,
            Some(c) => valid_colors.contains(&c.as_str()),
        }
    }
    fn valid_pid(&self) -> bool {
        let re = Regex::new("^[0-9]{9}$").expect("Failed to build Regex");
        match &self.pid {
            None => false,
            Some(pid) => re.is_match(pid.as_str()),
        }
    }
}

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|passport_str| parse_passport(passport_str))
        .collect()
}

fn parse_passport(passport_str: &str) -> Passport {
    let kv: Vec<&str> = passport_str
        .lines()
        .flat_map(|line| line.split(" "))
        .collect();
    let mut pass = Passport::new();
    for key_val in kv {
        let pair: Vec<&str> = key_val.split(":").collect();
        match *(pair.get(0).unwrap()) {
            "cid" => pass.cid = Some(String::from(*pair.get(1).unwrap())),
            "byr" => pass.byr = Some(str::parse(*pair.get(1).unwrap()).unwrap()),
            "iyr" => pass.iyr = Some(str::parse(*pair.get(1).unwrap()).unwrap()),
            "eyr" => pass.eyr = Some(str::parse(*pair.get(1).unwrap()).unwrap()),
            "hgt" => {
                pass.hgt_str = Some(str::parse(*pair.get(1).unwrap()).unwrap());
                pass.hgt = Height::parse(*pair.get(1).unwrap());
            }
            "hcl" => pass.hcl = Some(String::from(*pair.get(1).unwrap())),
            "ecl" => pass.ecl = Some(String::from(*pair.get(1).unwrap())),
            "pid" => pass.pid = Some(String::from(*pair.get(1).unwrap())),
            _ => panic!("Found passport code that doesn't match"),
        }
    }
    pass
}

#[aoc(day4, part1)]
fn count_valid_passports(input: &Vec<Passport>) -> usize {
    input.iter().filter(|pass| pass.has_fields()).count()
}

#[aoc(day4, part2)]
fn count_valid_data_passports(input: &Vec<Passport>) -> usize {
    input.iter().filter(|pass| pass.is_valid()).count()
}

#[cfg(test)]
#[test]
fn reads_passport() {
    let test_str = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                    hcl:#cfa07d byr:1929";
    let passport = Passport {
        byr: Some(1929),
        iyr: Some(2013),
        ecl: Some(String::from("amb")),
        cid: Some(String::from("350")),
        eyr: Some(2023),
        pid: Some(String::from("028048884")),
        hcl: Some(String::from("#cfa07d")),
        hgt: None,
        hgt_str: None,
    };
    assert_eq!(passport, parse_passport(test_str));
}

#[test]
fn test_valid_hgt() {
    let mut pass = Passport::new();
    pass.hgt = Height::parse("60in");
    assert_eq!(true, pass.valid_hgt())
}

#[test]
fn test_valid_hcl() {
    let mut pass = Passport::new();
    pass.hcl = Some(String::from("#19245f"));
    assert_eq!(true, pass.valid_hcl())
}

#[test]
fn test_valid_ecl() {
    let mut pass = Passport::new();
    pass.ecl = Some(String::from("amb"));
    assert_eq!(true, pass.valid_ecl())
}

#[test]
fn test_valid_pid() {
    let mut pass = Passport::new();
    pass.pid = Some(String::from("000000001"));
    assert_eq!(true, pass.valid_pid())
}

#[test]
fn invalid_test_battery() {
    let input_str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    assert_eq!(0, count_valid_data_passports(&parse_input_day4(input_str)));
}

#[test]
fn valid_test_battery() {
    let input_str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    assert_eq!(4, count_valid_data_passports(&parse_input_day4(input_str)));
}
