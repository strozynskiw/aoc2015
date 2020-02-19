extern crate num;

use std::fs;
use multimap::MultiMap;
use std::collections::HashMap;
use std::collections::HashSet;

const MOLECULE: &str = "ORnPBPMgArCaCaCaSiThCaCaSiThCaCaPBSiRnFArRnFArCaCaSiThCaCaSiThCaCaCaCaCaCaSiRnFYFArSiRnMgArCaSiRnPTiTiBFYPBFArSiRnCaSiRnTiRnFArSiAlArPTiBPTiRnCaSiAlArCaPTiTiBPMgYFArPTiRnFArSiRnCaCaFArRnCaFArCaSiRnSiRnMgArFYCaSiRnMgArCaCaSiThPRnFArPBCaSiRnMgArCaCaSiThCaSiRnTiMgArFArSiThSiThCaCaSiRnMgArCaCaSiRnFArTiBPTiRnCaSiAlArCaPTiRnFArPBPBCaCaSiThCaPBSiThPRnFArSiThCaSiThCaSiThCaPTiBSiRnFYFArCaCaPRnFArPBCaCaPBSiRnTiRnFArCaPRnFArSiRnCaCaCaSiThCaRnCaFArYCaSiRnFArBCaCaCaSiThFArPBFArCaSiRnFArRnCaCaCaFArSiRnFArTiRnPMgArF";

type Replacements = MultiMap<String, String>;

fn load_data(input: &str) -> Replacements {
    let mut replacements = MultiMap::new();
    let pairs: Vec<(&str, &str)> = input.lines().map(|l| -> (&str, &str) { let elements = l.split(" => ").collect::<Vec<&str>>(); (elements[0], elements[1])}).collect();
    pairs.iter().for_each(|p| replacements.insert(String::from(p.0), String::from(p.1)));
    replacements
}

fn part_1(input: &str) -> usize {
    let data = load_data(input);

    let mut molecules = HashSet::new();

    for (from, to) in data {
        for(index, _) in MOLECULE.match_indices(&from) {
            for value in &to {
                let molecule = [
                    &MOLECULE[0..index],
                    &value,
                    &MOLECULE[index + from.len()..],
                ]
                .concat();
                molecules.insert(molecule);
            }
        }
    }

    molecules.len()
}

fn part_2(input: &str) -> usize {
    let mut data = load_data(input);
    0
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
