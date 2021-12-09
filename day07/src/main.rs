use std::fs;

fn main() {
    let input = read_input("input");
    println!("{}", first(input.clone()));
    println!("{}", second(input));
}

fn first(input: String) -> i32 {
    let positions = parse_input(input);
    let max = positions.iter().max().unwrap().to_owned();
    let min = positions.iter().min().unwrap().to_owned();
    let mut min_cost = positions.iter().map(|p| (min-p).abs()).sum();
    for i in min+1..max+1 {
        let cost: i32 = positions.iter().map(|p| (i-p).abs()).sum();
        if cost < min_cost {
            min_cost = cost;
        }
    }
    return min_cost
}

fn second(input: String) -> i32 {
    let positions = parse_input(input);
    let max = positions.iter().max().unwrap().to_owned();
    let min = positions.iter().min().unwrap().to_owned();
    let mut min_cost = positions.iter().map(|p| cost(min,p.to_owned())).sum();
    for i in min+1..max+1 {
        let cost: i32 = positions.iter().map(|p| cost(i,p.to_owned())).sum();
        if cost < min_cost {
            min_cost = cost;
        }
    }
    return min_cost
}

fn cost(from: i32, to: i32) -> i32 {
    let distance = (from-to).abs();
    return distance*(distance+1)/2
}

fn parse_input(input: String) -> Vec<i32> {
    return input[0..input.len()-1].split(",").map(|n| n.parse().unwrap()).collect()
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
        let input = read_input("input_test");
        assert_eq!(first(input), 37);
    }
    #[test]
    fn test_2() {
        let input = read_input("input_test");
        assert_eq!(second(input), 168);
    }
}