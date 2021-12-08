use std::fs;

fn main() {
    let input = read_input();
    println!("1st: {}", first(input.clone()));
    println!("2nd: {}", second(input.clone()));
}

fn first(input: String) -> i32 {
    let mut numbers = Vec::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if numbers.len() < i+1 {
                numbers.push(0);
            }
            if c == '0' {
                numbers[i] -= 1;
            } else {
                numbers[i] += 1;
            }
        }
    }
    let mut gamma_str: String = "".to_string();
    let mut epsilon_str: String = "".to_string();
    for c in numbers {
        if c > 0 {
            gamma_str.push_str("1");
            epsilon_str.push_str("0");
        } else {
            gamma_str.push_str("0");
            epsilon_str.push_str("1");
        }
    }
    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap() as i32;
    let epsilon = isize::from_str_radix(&epsilon_str, 2).unwrap() as i32;
    return gamma * epsilon
}

fn second(input: String) -> i32 {
    let mut position = 0;
    let mut lines: Vec<_> = input.lines().map(|s| s.to_owned()).collect();
    let mut oxygene: i32 = 0;
    let mut co2: i32 = 0;
    loop {
        let value = eval(lines.clone(), position);
        lines = filter(lines.clone(), position, anti(value));
        if lines.len() == 1 {
            co2 = isize::from_str_radix(&lines[0][..], 2).unwrap() as i32;
            break
        }
        position += 1;
        if position > 20 {
            break
        }
    }
    lines = input.lines().map(|s| s.to_owned()).collect();
    position = 0;
    loop {
        let value = eval(lines.clone(), position);
        lines = filter(lines.clone(), position, value);
        if lines.len() == 1 {
            oxygene = isize::from_str_radix(&lines[0][..], 2).unwrap() as i32;
            break
        }
        position += 1;
        if position > 20 {
            break
        }
    }
    return co2 * oxygene
}

fn anti(value: char) -> char {
    if value == '0' {
        return '1';
    } else {
        return '0';
    }
}
fn eval(lines: Vec<String>, i: usize) -> char {
    let mut value = 0;
    for line in lines {
        if line.chars().nth(i).unwrap() == '1' {
            value += 1;
        } else {
            value -= 1;
        }
    }
    if value >= 0 {
        return '1';
    } 
    return '0';
}

fn filter(lines: Vec<String>, i: usize, c: char) -> Vec<String> {
    let mut new_lines = Vec::new();
    for line in lines {
        if line.chars().nth(i).unwrap() == c {
            new_lines.push(line)
        }
    }
    return new_lines
}

fn read_input() -> String {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_first() {
        let input: String = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".to_string();
        assert_eq!(first(input), 198);
    }

    #[test]
    fn test_second() {
        let input: String = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".to_string();
        assert_eq!(second(input), 230);
    }
}