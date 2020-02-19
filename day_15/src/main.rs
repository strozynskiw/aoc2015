#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::cmp::max;
use std::fs;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

fn parse_ingredients(input: &str) -> Ingredient {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<name>.+): capacity (?P<c>-?\d*\.{0,1}\d+), durability (?P<d>-?\d*\.{0,1}\d+), flavor (?P<f>-?\d*\.{0,1}\d+), texture (?P<t>-?\d*\.{0,1}\d+), calories (?P<ca>-?\d*\.{0,1}\d+)$").unwrap();
    }
    let cap = RE.captures(input).unwrap();
    Ingredient::new(
        cap["c"].parse::<i32>().unwrap(),
        cap["d"].parse::<i32>().unwrap(),
        cap["f"].parse::<i32>().unwrap(),
        cap["t"].parse::<i32>().unwrap(),
        cap["ca"].parse::<i32>().unwrap(),
    )
}

fn part_1(input: &str) -> i32 {
    let ingredients: Vec<Ingredient> = input.lines().map(|l| parse_ingredients(l)).collect();

    let spoons = 100;
    let mut best_result = 0;

    for i in 0..=spoons {
        for j in 0..=(spoons - i) {
            for k in 0..=(spoons - i - j) {
                for l in 0..=(spoons - i - j - k) {
                    let capacity = max(
                        0,
                        i * ingredients[0].capacity
                            + j * ingredients[1].capacity
                            + k * ingredients[2].capacity
                            + l * ingredients[3].capacity,
                    );
                    let durability = max(
                        0,
                        i * ingredients[0].durability
                            + j * ingredients[1].durability
                            + k * ingredients[2].durability
                            + l * ingredients[3].durability,
                    );
                    let flavor = max(
                        0,
                        i * ingredients[0].flavor
                            + j * ingredients[1].flavor
                            + k * ingredients[2].flavor
                            + l * ingredients[3].flavor,
                    );
                    let texture = max(
                        0,
                        i * ingredients[0].texture
                            + j * ingredients[1].texture
                            + k * ingredients[2].texture
                            + l * ingredients[3].texture,
                    );
                    best_result = max(best_result, capacity * durability * flavor * texture);
                }
            }
        }
    }

    best_result
}

fn part_2(input: &str) -> i32 {
    let ingredients: Vec<Ingredient> = input.lines().map(|l| parse_ingredients(l)).collect();

    let spoons = 100;
    let mut best_result = 0;

    for i in 0..=spoons {
        for j in 0..=spoons - i {
            for k in 0..=spoons - i - j {
                let l = spoons - i - j - k;
                let capacity = max(
                    0,
                    i * ingredients[0].capacity
                        + j * ingredients[1].capacity
                        + k * ingredients[2].capacity
                        + l * ingredients[3].capacity,
                );
                let durability = max(
                    0,
                    i * ingredients[0].durability
                        + j * ingredients[1].durability
                        + k * ingredients[2].durability
                        + l * ingredients[3].durability,
                );
                let flavor = max(
                    0,
                    i * ingredients[0].flavor
                        + j * ingredients[1].flavor
                        + k * ingredients[2].flavor
                        + l * ingredients[3].flavor,
                );
                let texture = max(
                    0,
                    i * ingredients[0].texture
                        + j * ingredients[1].texture
                        + k * ingredients[2].texture
                        + l * ingredients[3].texture,
                );
                let calories = i * ingredients[0].calories
                    + j * ingredients[1].calories
                    + k * ingredients[2].calories
                    + l * ingredients[3].calories;
                if calories == 500 {
                    best_result = max(best_result, capacity * durability * flavor * texture)
                }
            }
        }
    }

    best_result
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
