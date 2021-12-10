use std::fs;


fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(input_file: &str) -> u32 {
    let input = read_input(input_file);
    input.lines().map(|l| check_line(l)).sum()
}

fn second(input_file: &str) -> u64 {
    let input = read_input(input_file);
    let mut bounties = Vec::new();
    for line in input.lines() {
        if check_line(line) == 0 {
            bounties.push(complete_line(line));
        }
    }
    bounties.sort();
    return bounties[bounties.len()/2];
}
fn check_line(line: &str) -> u32 {
    let mut opened: Vec<char> = Vec::new();
    for c in line.chars() {
        if bounty(c) == 0 {
            opened.push(anti(c));
        } else {
            match opened.pop() {
                Some(x) => {
                    if c != x {
                        return bounty(c)
                    }}, 
                _ => break
            }
        }
    }
    return 0
}

fn complete_line(line: &str) -> u64 {
    let mut opened: Vec<char> = Vec::new();
    for c in line.chars() {
        if bounty(c) == 0 {
            opened.push(anti(c));
        } else {
            opened.pop().unwrap();
        }
    }
    let mut s = 0;
    for c in opened.iter().rev() {
        s = s*5 + score(c);
    }
    return s
}

fn anti(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => 'E',
    }
}

fn score(c: &char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
    }
}

fn bounty(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
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
        assert_eq!(first("input_test"), 26397);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 288957);
    }
}
