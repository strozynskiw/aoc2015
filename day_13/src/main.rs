#[macro_use]
extern crate lazy_static;
use permute::permutations_of;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type HappinessMap = HashMap<(String, String), i32>;

fn fill_happiness_map(input: &str, relations: &mut HappinessMap) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<p1>.*) would (?P<change>.*) (?P<value>\d+) happiness units by sitting next to (?P<p2>.*).$").unwrap();
    }

    for line in input.lines() {
        let cap = RE.captures(line).unwrap();
        let value = match &cap["change"] {
            "lose" => -(cap["value"].parse::<i32>().unwrap()),
            "gain" => (cap["value"].parse::<i32>().unwrap()),
            _ => 0,
        };
        relations.insert((String::from(&cap["p1"]), String::from(&cap["p2"])), value);
    }
}

fn get_guest_names(guests: &mut HashSet<String>, relations: &HappinessMap) {
    for k in relations.keys() {
        guests.insert(String::from(&k.0));
    }
}

fn compute_happiness_change(relations: &HappinessMap) -> i32 {
    let mut guests: HashSet<String> = HashSet::new();
    get_guest_names(&mut guests, &relations);
    let guest_vec = guests.iter().collect::<Vec<&String>>();
    let mut highest_change = 0;
    for permutation in permutations_of(&guest_vec) {
        let mut sum: i32 = 0;
        let v: Vec<String> = permutation.map(|&x| x.clone()).collect();

        sum += relations[&(v[0].clone(), v[v.len() - 1].clone())];
        sum += relations[&(v[v.len() - 1].clone(), v[0].clone())];

        for i in 0..v.len() - 1 {
            sum += relations[&(v[i].clone(), v[i + 1].clone())];
            sum += relations[&(v[i + 1].clone(), v[i].clone())];
        }
        highest_change = max(highest_change, sum);
    }
    highest_change
}

fn part_1(input: &str) -> i32 {
    let mut relations: HappinessMap = HappinessMap::new();
    fill_happiness_map(input, &mut relations);
    compute_happiness_change(&relations)
}

fn part_2(input: &str) -> i32 {
    let mut relations: HappinessMap = HappinessMap::new();
    fill_happiness_map(input, &mut relations);
    let mut guests: HashSet<String> = HashSet::new();
    get_guest_names(&mut guests, &relations);

    for guest in guests {
        relations.insert((String::from("me"), (String::from(&guest))), 0);
        relations.insert((String::from(&guest), (String::from("me"))), 0);
    }

    compute_happiness_change(&relations)
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let r1 = part_1(&content);
    let r2 = part_2(&content);

    println!("Result1: {}", r1);
    println!("Result2: {}", r2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {}
}
