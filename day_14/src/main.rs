#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::cmp::min;
use std::fs;

struct Reindeer {
    travel_speed: i32,
    travel_time: i32,
    resting_time: i32,
    distance: i32,
    points: i32,
}

impl Reindeer {
    fn new(travel_speed: i32, travel_time: i32, resting_time: i32) -> Self {
        Reindeer {
            travel_speed,
            travel_time,
            resting_time,
            distance: 0,
            points: 0,
        }
    }
}

fn parse_reindeer(input: &str) -> Reindeer {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<r>.*) can fly (?P<speed>\d+) km/s for (?P<time>\d+) seconds, but then must rest for (?P<rtime>\d+) seconds.$").unwrap();
    }

    let cap = RE.captures(input).unwrap();
    Reindeer::new(
        cap["speed"].parse::<i32>().unwrap(),
        cap["time"].parse::<i32>().unwrap(),
        cap["rtime"].parse::<i32>().unwrap(),
    )
}

fn compute_distance(r: &Reindeer, time: i32) -> i32 {
    let full_cycles = time / (r.travel_time + r.resting_time);
    let full_cycles_time = full_cycles * (r.travel_time + r.resting_time);
    let last_period = time - full_cycles_time;
    (full_cycles * r.travel_time + min(r.travel_time, last_period)) * r.travel_speed
}

fn part_1(input: &str, time: i32) -> i32 {
    let mut reindeers: Vec<Reindeer> = input.lines().map(|line| parse_reindeer(line)).collect();

    emulate_reindeers(&mut reindeers, time);
    reindeers.iter().map(|r| r.distance).max().unwrap()
}

fn emulate_reindeers(reindeers: &mut Vec<Reindeer>, time: i32) {
    for t in 1..=time {
        reindeers
            .iter_mut()
            .for_each(|r| r.distance = compute_distance(r, t));
        let highest_distance = reindeers.iter().map(|r| r.distance).max().unwrap();
        reindeers.iter_mut().for_each(|r| {
            if highest_distance == r.distance {
                r.points += 1;
            }
        });
    }
}

fn part_2(input: &str, time: i32) -> i32 {
    let mut reindeers: Vec<Reindeer> = input.lines().map(|line| parse_reindeer(line)).collect();
    emulate_reindeers(&mut reindeers, time);

    reindeers.iter().map(|r| r.points).max().unwrap()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let r1 = part_1(&content, 2503);
    let r2 = part_2(&content, 2503);

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
