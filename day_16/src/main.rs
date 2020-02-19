#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn is_the_sue(input: &str, the_sue: &HashMap<String, i32>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Sue (?P<n>\d+): (?P<p1>.+): (?P<v1>\d+), (?P<p2>.+): (?P<v2>\d+), (?P<p3>.+): (?P<v3>\d+)$").unwrap();
    }

    let cap = RE.captures(input).unwrap();

    the_sue[&cap["p1"]] == cap["v1"].parse().unwrap()
        && the_sue[&cap["p2"]] == cap["v2"].parse().unwrap()
        && the_sue[&cap["p3"]] == cap["v3"].parse().unwrap()
}

fn check_match(key: &str, sue_val: i32, val: i32) -> bool {
    match key {
        "cats" => val > sue_val,
        "trees" => val > sue_val,
        "pomeranians" => val < sue_val,
        "goldfish" => val < sue_val,
        _ => val == sue_val,
    }
}

fn is_the_real_sue(input: &str, the_sue: &HashMap<String, i32>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Sue (?P<n>\d+): (?P<p1>.+): (?P<v1>\d+), (?P<p2>.+): (?P<v2>\d+), (?P<p3>.+): (?P<v3>\d+)$").unwrap();
    }

    let cap = RE.captures(input).unwrap();

    check_match(&cap["p1"], the_sue[&cap["p1"]], cap["v1"].parse().unwrap())
        && check_match(&cap["p2"], the_sue[&cap["p2"]], cap["v2"].parse().unwrap())
        && check_match(&cap["p3"], the_sue[&cap["p3"]], cap["v3"].parse().unwrap())
}

fn part_1(input: &str) -> i32 {
    let mut the_sue: HashMap<String, i32> = HashMap::new();
    the_sue.insert(String::from("children"), 3);
    the_sue.insert(String::from("cats"), 7);
    the_sue.insert(String::from("samoyeds"), 2);
    the_sue.insert(String::from("pomeranians"), 3);
    the_sue.insert(String::from("akitas"), 0);
    the_sue.insert(String::from("vizslas"), 0);
    the_sue.insert(String::from("goldfish"), 5);
    the_sue.insert(String::from("trees"), 3);
    the_sue.insert(String::from("cars"), 2);
    the_sue.insert(String::from("perfumes"), 1);

    for (i, l) in input.lines().enumerate() {
        if is_the_sue(l, &the_sue) {
            return i as i32 + 1;
        }
    }
    return 0;
}

fn part_2(input: &str) -> i32 {
    let mut the_sue: HashMap<String, i32> = HashMap::new();
    the_sue.insert(String::from("children"), 3);
    the_sue.insert(String::from("cats"), 7);
    the_sue.insert(String::from("samoyeds"), 2);
    the_sue.insert(String::from("pomeranians"), 3);
    the_sue.insert(String::from("akitas"), 0);
    the_sue.insert(String::from("vizslas"), 0);
    the_sue.insert(String::from("goldfish"), 5);
    the_sue.insert(String::from("trees"), 3);
    the_sue.insert(String::from("cars"), 2);
    the_sue.insert(String::from("perfumes"), 1);

    for (i, l) in input.lines().enumerate() {
        if is_the_real_sue(l, &the_sue) {
            return i as i32 + 1;
        }
    }
    return 0;
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let r1 = part_1(&content);
    let r2 = part_2(&content);

    println!("Result1: {}", r1);
    println!("Result2: {}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {}
}
