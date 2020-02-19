use itertools::Itertools;
use std::fs;

fn calc(input: &str, times: u64) -> usize {
    let mut output_string: String = input.to_string();
    for _ in 0..times {
        let output: Vec<String> = output_string
            .chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_, r)| r.collect())
            .collect();

        output_string = "".to_string();

        //Create new string
        for x in output {
            output_string.push_str(&format!("{}{}", x.len(), x.chars().next().unwrap()));
        }
    }

    output_string.len()
}

fn main() {
    let r1 = calc("1113122113", 40);
    let r2 = calc("1113122113", 50);

    println!("Result1: {}", r1);
    println!("Result2: {}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(2, calc("1", 1));
        assert_eq!(2, calc("11", 1));
        assert_eq!(4, calc("21", 1));
        assert_eq!(6, calc("1211", 1));
    }
}
