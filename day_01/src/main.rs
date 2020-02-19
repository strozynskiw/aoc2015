use std::fs;
fn part_1(_input: &str) -> i32 {
    let mut floor = 0;
    for c in _input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn part_2(_input: &str) -> i32 {
    let mut floor = 0;
    let mut i = 0;
    for c in _input.chars() {
        if c == '(' {
            floor += 1
        } else if c == ')' {
            floor -= 1
        }

        i += 1;
        if floor == -1 {
            break;
        }
    }
    i
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
        assert_eq!(2, part_1("(((())()"));
        assert_eq!(3, part_1("(()(()("));
        assert_eq!(-3, part_1(")())())"));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(1, part_2(")"));
        assert_eq!(5, part_2("()())"));
    }
}
