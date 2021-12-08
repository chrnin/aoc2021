use std::fs;

fn main() {
    let input = read_input("input");
    println!("first: {}", simul(input.clone(), 80));
    println!("second: {}", simul(input.clone(), 256));
}

fn simul(input: String, days: usize) -> usize {
    let mut population = build_population(input);
    while population[9] < days {
        old(&mut population);
    }
    return count(population);
}

fn count(population: [usize;10]) -> usize {
    return population[0..9].iter().sum()
}

fn old(population: &mut [usize;10]) {
    let p0 = population[(population[9])%7];
    population[(population[9])%7] += population[7];
    population[7] = population[8];
    population[8] = p0;
    population[9] += 1;
}

fn build_population(input: String) -> [usize;10] {
    let mut population: [usize;10] = [0;10];
    let fishes: Vec<i32> = input[0..input.len()-1].split(",").map(|f| {
        f.parse().unwrap()}
    ).collect();
    for f in fishes {
        population[f as usize] += 1;
    }
    return population
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
        assert_eq!(simul(input, 18), 26);
    }
    #[test]
    fn test_2() {
        let input = read_input("input_test");
        assert_eq!(simul(input, 256), 26984457539);
    }
}