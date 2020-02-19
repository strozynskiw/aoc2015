use itertools::Itertools;
use std::fs;

fn part_1(input: &str, total_volume: i32) -> usize {
    let mut containers: Vec<i32> = Vec::new();
    input
        .lines()
        .for_each(|l| containers.push(l.parse::<i32>().unwrap()));

    let mut val = 0;

    for i in 1..containers.len() {
        println!("Combinations {}", i);
        let it = containers.iter().combinations(i);
        val += it
            .filter(|v| v.iter().fold(0, |acc, v| acc + *v) == total_volume)
            .count()
    }

    val
}

fn part_2(input: &str, total_volume: i32) -> usize {
    let mut containers: Vec<i32> = Vec::new();
    input
        .lines()
        .for_each(|l| containers.push(l.parse::<i32>().unwrap()));

    for i in 1..containers.len() {
        println!("Combinations {}", i);
        let it = containers.iter().combinations(i);
        let correct_combinations = it
            .filter(|v| v.iter().fold(0, |acc, v| acc + *v) == total_volume)
            .count();
        if correct_combinations > 0 {
            return correct_combinations;
        }
    }

    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let r1 = part_1(&content, 150);
    let r2 = part_2(&content, 150);

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
