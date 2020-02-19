use std::fs;

fn count_removed_chars(input: &str) -> usize {
    let mut remove_chars = 2;

    let mut chars = input.chars().peekable();

    while chars.peek().is_some() {
        let c = chars.next().unwrap();

        if c == '\\' {
            let c2 = chars.next().unwrap();
            if c2 == 'x' {
                remove_chars += 2;
                chars.nth(1);
            }
            remove_chars += 1;
        }
    }
    remove_chars
}

fn count_removed_chars_encoded(input: &str) -> usize {
    input.chars().fold(0, |acc, c| {
        acc + match c {
            '"' => 1,
            '\\' => 1,
            _ => 0,
        }
    })
}

fn part_1(input: &str) -> usize {
    input.lines().fold(0, |acc, l| acc + count_removed_chars(l))
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, l| acc + 2 + count_removed_chars_encoded(l))
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
    fn test_part_1() {
        assert_eq!(2, part_1("\"\""));
        assert_eq!(2, part_1("\"abc\""));
        assert_eq!(3, part_1("\"aaa\\\"aaa\""));
        assert_eq!(5, part_1("\"aaa\\x27aaa\""));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(4, part_2("\"\""));
        assert_eq!(4, part_2("\"abc\""));
        assert_eq!(5, part_2("\"aaa\\\"aaa\""));
        assert_eq!(7, part_2("\"aaa\\x27aaa\""));
    }
}
