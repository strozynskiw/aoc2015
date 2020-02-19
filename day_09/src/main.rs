#[macro_use]
extern crate lazy_static;

use permute::permutations_of;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type DistancesType = HashMap<(String, String), u32>;

fn parse_line(line: &str, cities: &mut HashSet<String>, distances: &mut DistancesType) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<c1>.*) to (?P<c2>.*) = (?P<dist>\d+)$").unwrap();
    }

    let cap = RE.captures(line).unwrap();
    cities.insert(String::from(&cap["c1"]));
    cities.insert(String::from(&cap["c2"]));
    distances.insert(
        (String::from(&cap["c1"]), String::from(&cap["c2"])),
        cap["dist"].parse().unwrap(),
    );
}

fn get_distance(c1: &String, c2: &String, distances: &DistancesType) -> u32 {
    if distances.contains_key(&(c1.clone(), c2.clone())) {
        return *distances.get(&(c1.clone(), c2.clone())).unwrap();
    } else if distances.contains_key(&(c2.clone(), c1.clone())) {
        return *distances.get(&(c2.clone(), c1.clone())).unwrap();
    }
    panic!("Could't find distance");
}

fn part_1(input: &str) -> u32 {
    let mut cities: HashSet<String> = HashSet::new();
    let mut distances: DistancesType = HashMap::new();

    input
        .lines()
        .for_each(|x| parse_line(x, &mut cities, &mut distances));
    let cities_vec: Vec<&String> = cities.iter().collect();

    let mut shortest_distance = u32::max_value();

    for permutation in permutations_of(&cities_vec) {
        let mut sum: u32 = 0;
        let mut peekable = permutation.peekable();
        for _ in 0..cities_vec.len() - 1 {
            sum += get_distance(
                peekable.next().unwrap(),
                peekable.peek().unwrap(),
                &distances,
            )
        }
        shortest_distance = cmp::min(sum, shortest_distance);
    }

    shortest_distance
}

fn part_2(input: &str) -> u32 {
    let mut cities: HashSet<String> = HashSet::new();
    let mut distances: DistancesType = HashMap::new();

    input
        .lines()
        .for_each(|x| parse_line(x, &mut cities, &mut distances));
    let cities_vec: Vec<&String> = cities.iter().collect();

    let mut longest_distance = u32::min_value();

    for permutation in permutations_of(&cities_vec) {
        let mut sum: u32 = 0;
        let mut peekable = permutation.peekable();
        for _ in 0..cities_vec.len() - 1 {
            sum += get_distance(
                peekable.next().unwrap(),
                peekable.peek().unwrap(),
                &distances,
            )
        }
        longest_distance = cmp::max(sum, longest_distance);
    }

    longest_distance
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
