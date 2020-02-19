//This solution is a mess... I need to refactor that later

use std::collections::HashMap;
use std::fs;

fn execute_rule(line: &str, wires: &mut HashMap<String, u16>) {
    let rule = parse_rule(&line);

    match &rule.0 as &str {
        "AND" => {
            let a = match wires.contains_key(&rule.1) {
                true => *wires.get(&rule.1).unwrap(),
                false => match rule.1.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            let b = match wires.contains_key(&rule.2) {
                true => *wires.get(&rule.2).unwrap(),
                false => match rule.2.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            wires.insert(rule.3, a & b);
        }
        "OR" => {
            let a = match wires.contains_key(&rule.1) {
                true => *wires.get(&rule.1).unwrap(),
                false => match rule.1.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            let b = match wires.contains_key(&rule.2) {
                true => *wires.get(&rule.2).unwrap(),
                false => match rule.2.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            wires.insert(rule.3, a | b);
        }
        "LSHIFT" => {
            let a = match wires.contains_key(&rule.1) {
                true => *wires.get(&rule.1).unwrap(),
                false => match rule.1.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            let b = match wires.contains_key(&rule.2) {
                true => *wires.get(&rule.2).unwrap(),
                false => match rule.2.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            wires.insert(rule.3, a << b);
        }
        "RSHIFT" => {
            let a = match wires.contains_key(&rule.1) {
                true => *wires.get(&rule.1).unwrap(),
                false => match rule.1.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            let b = match wires.contains_key(&rule.2) {
                true => *wires.get(&rule.2).unwrap(),
                false => match rule.2.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            wires.insert(rule.3, a >> b);
        }
        "NOT" => {
            let a = match wires.contains_key(&rule.1) {
                true => *wires.get(&rule.1).unwrap(),
                false => match rule.1.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };

            wires.insert(rule.3, !a);
        }
        "" => {
            let a = match wires.contains_key(&rule.1) {
                true => *wires.get(&rule.1).unwrap(),
                false => match rule.1.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return,
                },
            };
            wires.entry(rule.3).or_insert(a);
        }
        &_ => panic!("Sth is wrong!"),
    }
}

fn parse_rule(line: &str) -> (String, String, String, String) {
    let keys: Vec<&str> = line.split_whitespace().collect();

    if line.contains("AND")
        || line.contains("OR")
        || line.contains("LSHIFT")
        || line.contains("RSHIFT")
    {
        let order = String::from(keys[1]);
        let a = String::from(keys[0]);
        let b = String::from(keys[2]);
        let x = String::from(keys[4]);
        (order, a, b, x)
    } else if line.contains("NOT") {
        let order = String::from(keys[0]);
        let a = String::from(keys[1]);
        let b = String::from(keys[1]);
        let x = String::from(keys[3]);
        (order, a, b, x)
    } else {
        let order = String::from("");
        let a = String::from(keys[0]);
        let b = String::from(keys[0]);
        let x = String::from(keys[2]);
        (order, a, b, x)
    }
}

fn part_1(input: &str) -> u16 {
    let mut wires: HashMap<String, u16> = HashMap::new();
    while wires.get("a").is_none() {
        for line in input.lines() {
            execute_rule(line, &mut wires)
        }
    }
    *wires.get("a").unwrap()
}

fn part_2(input: &str) -> u16 {
    let mut wires: HashMap<String, u16> = HashMap::new();
    wires.insert(String::from("b"), part_1(input));
    while wires.get("a").is_none() {
        for line in input.lines() {
            execute_rule(line, &mut wires)
        }
    }
    *wires.get("a").unwrap()
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
        assert_eq!(4, part_1("1 -> a\na LSHIFT 2 -> a"));
        assert_eq!(2, part_1("8 -> a\na RSHIFT 2 -> a"));
        assert_eq!(0, part_1("1 -> a\n2 -> b\n a AND b -> a"));
        assert_eq!(3, part_1("1 -> a\n2 -> b\n a OR b -> a"));
        assert_eq!(!61680, part_1("61680 -> a\nNOT a -> a"));
    }

    #[test]
    fn test_part_2() {}
}
