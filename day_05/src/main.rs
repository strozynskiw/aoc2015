use std::fs;

fn check_vowels(input: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    input
        .chars()
        .fold(0, |acc, x| acc + vowels.contains(&x) as u32)
        >= 3
}

fn check_double_letters(input: &str) -> bool {
    input
        .as_bytes()
        .windows(2)
        .any(|window| window[0] == window[1])
}

fn check_substrings(input: &str) -> bool {
    !(input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy"))
}

fn part_1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        acc + (check_vowels(&line) && check_double_letters(&line) && check_substrings(&line)) as u32
    })
}

fn check_pairs_with_letter_between(input: &str) -> bool {
    input
        .as_bytes()
        .windows(3)
        .any(|window| window[0] == window[2])
}

fn check_double_pairs_no_overlapping(input: &str) -> bool {
    input
        .as_bytes()
        .windows(2)
        .enumerate()
        .any(|(index, first)| {
            input
                .as_bytes()
                .windows(2)
                .skip(index + 2)
                .any(|second| first[0] == second[0] && first[1] == second[1])
        })
}

fn part_2(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        acc + (check_pairs_with_letter_between(&line) && check_double_pairs_no_overlapping(&line))
            as u32
    })
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
        assert_eq!(1, part_1("aaa"));
        assert_eq!(0, part_1("abaa"));
        assert_eq!(1, part_1("aihghghhge"));
        assert_eq!(0, part_1("aihghghhgoucd"));
        assert_eq!(1, part_1("ugknbfddgicrmopn"));
        assert_eq!(0, part_1("jchzalrnumimnmhp"));
        assert_eq!(0, part_1("haegwjzuvuyypxyu"));
        assert_eq!(0, part_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1, part_2("qjhvhtzxzqqjkmpb"));
        assert_eq!(1, part_2("xxyxx"));
        assert_eq!(0, part_2("uurcxstgmygtbstg"));
        assert_eq!(0, part_2("ieodomkazucvgmuy"));
    }
}
