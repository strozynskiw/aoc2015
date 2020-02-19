use std::collections::HashSet;
use std::fs;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }
    fn left(&mut self) {
        self.x -= 1;
    }
    fn right(&mut self) {
        self.x += 1;
    }
    fn up(&mut self) {
        self.y -= 1;
    }
    fn down(&mut self) {
        self.y += 1;
    }
    fn make_move(&mut self, c: char) {
        match c {
            '^' => self.up(),
            'v' => self.down(),
            '<' => self.left(),
            '>' => self.right(),
            _ => (),
        }
    }
}

fn part_1(_input: &str) -> usize {
    let mut point = Point::new();
    let mut positions = HashSet::new();

    positions.insert((point.x, point.y));

    for c in _input.chars() {
        point.make_move(c);
        positions.insert((point.x, point.y));
    }



    positions.len()
}

fn part_2(_input: &str) -> usize {
    let mut santa_position = Point::new();
    let mut robot_position = Point::new();
    let mut positions = HashSet::new();

    positions.insert((0, 0));

    for (index, c) in _input.chars().enumerate() {
        match index % 2 {
            0 => {
                santa_position.make_move(c);
                positions.insert((santa_position.x, santa_position.y));
            }
            1 => {
                robot_position.make_move(c);
                positions.insert((robot_position.x, robot_position.y));
            }
            _ => (),
        }
    }

    positions.len()
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
        assert_eq!(2, part_1(">"));
        assert_eq!(4, part_1("^>v<"));
        assert_eq!(2, part_1("^v^v^v^v^v"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3, part_2("^v"));
        assert_eq!(3, part_2("^>v<"));
        assert_eq!(11, part_2("^v^v^v^v^v"));
    }
}
