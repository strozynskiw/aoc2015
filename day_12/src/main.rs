use serde_json::Value;
use std::fs;

fn process_1(v: &Value) -> i64 {
    let mut acc = 0;
    match v {
        Value::Number(number) => acc += number.as_i64().unwrap(),
        Value::Array(array) => acc += array.iter().map(|item| process_1(item)).sum::<i64>(),
        Value::Object(object) => acc += object.values().map(|item| process_1(item)).sum::<i64>(),
        _ => (),
    };
    acc
}

fn process_2(v: &Value) -> i64 {
    let mut acc = 0;
    match v {
        Value::Number(number) => acc += number.as_i64().unwrap(),
        Value::Array(array) => acc += array.iter().map(|item| process_2(item)).sum::<i64>(),
        Value::Object(object) => {
            if object.values().any(|v| match v {
                Value::String(s) => s.contains("red"),
                _ => false,
            }) {
                acc = 0;
                return acc;
            }

            acc += object.values().map(|item| process_2(item)).sum::<i64>();
        }
        _ => (),
    };
    acc
}

fn part_1(v: &Value) -> i64 {
    process_1(v)
}

fn part_2(v: &Value) -> i64 {
    process_2(v)
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let j: Value = serde_json::from_str(content).unwrap();

    let r1 = part_1(&j);
    let r2 = part_2(&j);

    println!("Result1: {}", r1);
    println!("Result2: {}", r2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(6, part_1(&serde_json::from_str(r"[1,2,3]").unwrap()));
        assert_eq!(
            6,
            part_1(&serde_json::from_str(r#"{"a":2,"b":4}"#).unwrap())
        );
        assert_eq!(3, part_1(&serde_json::from_str(r"[[[3]]]").unwrap()));
        assert_eq!(
            3,
            part_1(&serde_json::from_str(r#"{"a":{"b":4},"c":-1}"#).unwrap())
        );
        assert_eq!(0, part_1(&serde_json::from_str(r#"{"a":[-1,1]}"#).unwrap()));
        assert_eq!(0, part_1(&serde_json::from_str(r#"[-1,{"a":1}]"#).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(6, part_2(&serde_json::from_str(r"[1,2,3]").unwrap()));
        assert_eq!(
            4,
            part_2(&serde_json::from_str(r#"[1,{"c":"red","b":2},3]"#).unwrap())
        );
        assert_eq!(3, part_2(&serde_json::from_str(r"[[[3]]]").unwrap()));
        assert_eq!(
            3,
            part_2(&serde_json::from_str(r#"{"a":{"b":4},"c":-1}"#).unwrap())
        );
        assert_eq!(
            0,
            part_2(&serde_json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap())
        );
        assert_eq!(6, part_2(&serde_json::from_str(r#"[1,"red",5]"#).unwrap()));
    }
}
