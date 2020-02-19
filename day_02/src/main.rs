use std::fs;

fn get_dims(line: &str) -> (u32, u32, u32) {
    let mut dims: Vec<u32> = line.split('x').map(|dim| dim.parse().unwrap()).collect();
    dims.sort();
    (dims[0], dims[1], dims[2])
}

fn part_1(_input: &str) -> u32 {
    let mut area: u32 = 0;
    for line in _input.lines() {
        let (a, b, c) = get_dims(line);
        area += 3 * a * b + 2 * b * c + 2 * c * a;
    }
    area
}

fn part_2(_input: &str) -> u32 {
    let mut ribbon: u32 = 0;
    for line in _input.lines() {
        let (a, b, c) = get_dims(line);
        ribbon += 2 * a + 2 * b + a * b * c;
    }
    ribbon
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
        assert_eq!(58, part_1("2x3x4"));
        assert_eq!(43, part_1("1x1x10"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(34, part_2("2x3x4"));
        assert_eq!(14, part_2("1x1x10"));
    }
}
