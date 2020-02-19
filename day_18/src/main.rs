extern crate num;

use num::clamp;
use std::fs;

type LightMap = Vec<Vec<bool>>;

fn load_data(input: &str) -> LightMap {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect::<LightMap>()
}

fn get_value(pos_x: usize, pos_y: usize, data: &LightMap) -> bool {
    let limit_x_down: i32 = clamp(pos_x as i32 - 1, 0, data.len() as i32 - 1);
    let limit_x_up: i32 = clamp(pos_x as i32 + 1, 0, data.len() as i32 - 1);

    let limit_y_down: i32 = clamp(pos_y as i32 - 1, 0, data[0].len() as i32 - 1);
    let limit_y_up: i32 = clamp(pos_y as i32 + 1, 0, data[0].len() as i32 - 1);

    let mut value = 0;
    for i in limit_x_down..=limit_x_up {
        for j in limit_y_down..=limit_y_up {
            if i == pos_x as i32 && j == pos_y as i32 {
                continue;
            }
            value += data[i as usize][j as usize] as i32;
        }
    }

    match data[pos_x][pos_y] {
        true => match value {
            2..=3 => true,
            _ => false,
        },
        false => match value {
            3 => true,
            _ => false,
        },
    }
}

fn make_step(data: &LightMap) -> LightMap {
    data.iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, _)| get_value(x, y, data))
                .collect::<Vec<bool>>()
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let mut data = load_data(input);

    for _ in 0..100 {
        data = make_step(&data);
    }

    data.iter().flatten().map(|i| *i as usize).sum()
}

fn brake_light(data: &mut LightMap) {
    let len_x = data.len() - 1;
    let len_y = data[0].len() - 1;

    data[0 as usize][0 as usize] = true;
    data[len_x as usize][0 as usize] = true;
    data[0 as usize][len_y as usize] = true;
    data[len_x as usize][len_y as usize] = true;
}

fn part_2(input: &str) -> usize {
    let mut data = load_data(input);

    brake_light(&mut data);

    for _ in 0..100 {
        data = make_step(&data);
        brake_light(&mut data)
    }

    data.iter().flatten().map(|i| *i as usize).sum()
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
