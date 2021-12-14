use std::collections::HashMap;
use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(input_file: &str) -> u64 {
    let (polystring, insertions) = parse_input(input_file);
    let mut polymer = Polymer::new();
    polymer.parse(polystring);
    for _ in 0..10 {
        polymer.polymerize(&insertions);
    }
    let (min, max) = min_max(&polymer.chars);
    return max - min
}

fn second(input_file: &str) -> u64 {
    let (polystring, insertions) = parse_input(input_file);
    let mut polymer = Polymer::new();
    polymer.parse(polystring);

    for _ in 0..40 {
        polymer.polymerize(&insertions);
    }

    let (min, max) = min_max(&polymer.chars);

    return max - min
}

fn min_max(count: &HashMap<char, u64>) -> (u64, u64) {
    let mut min = u64::MAX;
    let mut max = 0;
    for (_, v) in count {
        if v > &max {
            max = *v
        };
        if v < &min {
            min = *v
        };
    }
    (min, max)
}

#[derive(Debug)]
struct Polymer {
    chars: HashMap<char, u64>,
    pairs: HashMap<[char; 2], u64>,
}

impl Polymer {
    fn new() -> Polymer {
        Polymer {
            chars: HashMap::new(),
            pairs: HashMap::new(),
        }
    }

    fn parse(&mut self, polymer: Vec<char>) {
        let mut pairs = HashMap::new();
        for i in 0..polymer.len() - 1 {
            let pair = [polymer[i], polymer[i + 1]];
            *pairs.entry(pair).or_insert(0) += 1;
        }
        self.pairs = pairs;

        let mut chars = HashMap::new();
        for i in polymer {
            *chars.entry(i).or_insert(0) += 1;
        }
        self.chars = chars;
    }
    fn polymerize(&mut self, insertions: &HashMap<[char; 2], char>) {
        let mut new_pairs = HashMap::new();
        for (pair, value) in &self.pairs.clone() {
            let pair_a = [pair[0], insertions[pair]];
            let pair_b = [insertions[pair], pair[1]];
            *new_pairs.entry(pair_a).or_insert(0) += value;
            *new_pairs.entry(pair_b).or_insert(0) += value;
            *self.chars.entry(insertions[pair]).or_insert(0) += value;
        }
        self.pairs = new_pairs;
    }
}

fn parse_input(filename: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    let input = read_input(filename);
    let mut lines = input.lines();
    let polymer = lines.next().unwrap().chars().collect();
    let _ = lines.next(); // empty line in input
    let mut insertions = HashMap::new();
    loop {
        let line = lines.next();
        match line {
            Some(i) => {
                let values: Vec<&str> = i.split(" -> ").collect();
                let pair: Vec<char> = values[0].chars().collect();
                let insert = values[1].chars().next().unwrap();
                insertions.insert([pair[0], pair[1]], insert);
            }
            None => break,
        }
    }
    (polymer, insertions)
}
fn read_input(filename: &str) -> String {
    let content = fs::read_to_string(filename).expect("lecture du fichier impossible");
    return content;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first("input_test"), 1588);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 2188189693529);
    }
}
