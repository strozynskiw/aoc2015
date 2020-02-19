#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone)]
enum Order {
    On,
    Off,
    Toggle,
}

#[derive(Copy, Clone)]
struct Command {
    order: Order,
    first: (u32, u32),
    second: (u32, u32),
}

fn parse_command(line: &str) -> Command {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<c>.*) (?P<fx>\d+),(?P<fy>\d+) through (?P<sx>\d+),(?P<sy>\d+)$")
                .unwrap();
    }
    let cap = RE.captures(line).unwrap();

    let get_cmd = |x: &str| match x {
        "turn on" => Order::On,
        "turn off" => Order::Off,
        "toggle" => Order::Toggle,
        _ => panic!("Parsing error"),
    };

    let cmd = get_cmd(&cap["c"]);
    Command {
        order: cmd,
        first: (cap["fx"].parse().unwrap(), cap["fy"].parse().unwrap()),
        second: (cap["sx"].parse().unwrap(), cap["sy"].parse().unwrap()),
    }
}

fn apply_command_to_grid(command: &Command, grid: &mut HashMap<(u32, u32), bool>) {
    assert_eq!(
        true,
        command.first.0 <= command.second.0 && command.first.1 <= command.second.1
    );

    for i in command.first.0..=command.second.0 {
        for j in command.first.1..=command.second.1 {
            grid.entry((i, j)).or_insert(false);

            *grid.get_mut(&(i, j)).unwrap() = match command.order {
                Order::On => true,
                Order::Off => false,
                Order::Toggle => !grid.get(&(i, j)).unwrap(),
            };
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut grid: HashMap<(u32, u32), bool> = HashMap::new();

    let mut process_line = |line: &str| {
        let command = parse_command(&line);
        apply_command_to_grid(&command, &mut grid);
    };

    input.lines().for_each(|x| process_line(&x));
    grid.iter().filter(|&(_k, v)| *v).count()
}

fn apply_brightness_command_to_grid(command: &Command, grid: &mut HashMap<(u32, u32), i32>) {
    assert_eq!(
        true,
        command.first.0 <= command.second.0 && command.first.1 <= command.second.1
    );

    for i in command.first.0..=command.second.0 {
        for j in command.first.1..=command.second.1 {
            grid.entry((i, j)).or_insert(0);

            *grid.get_mut(&(i, j)).unwrap() += match command.order {
                Order::On => 1,
                Order::Off => -1,
                Order::Toggle => 2,
            };

            if *grid.get_mut(&(i, j)).unwrap() < 0 {
                *grid.get_mut(&(i, j)).unwrap() = 0
            };
        }
    }
}

fn part_2(input: &str) -> i32 {
    let mut grid: HashMap<(u32, u32), i32> = HashMap::new();

    let mut process_line = |line: &str| {
        let command = parse_command(&line);
        apply_brightness_command_to_grid(&command, &mut grid);
    };

    input.lines().for_each(|x| process_line(&x));
    grid.iter().fold(0 as i32, |acc, x| acc + x.1)
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    println!("Result1: {}", part_1(&content));
    println!("Result2: {}", part_2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(1000 * 1000, part_1("turn on 0,0 through 999,999"));
        assert_eq!(1000, part_1("turn on 0,0 through 0,999"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1, part_2("turn on 0,0 through 0,0"));
        assert_eq!(2 * 1000 * 1000, part_2("toggle 0,0 through 999,999"));
    }
}
