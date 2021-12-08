use std::collections::HashMap;
use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(input_file: &str) -> i32 {
    let input = read_input(input_file);
    let mut count = 0;
    for line in input.lines() {
        let l: Vec<&str> = line.split(" | ").collect();
        let digits: Vec<&str> = l[1].split(" ").collect();
        for d in digits {
            if d.len() == 2 || d.len() == 3 || d.len() == 4 || d.len() == 7 {
                count += 1
            }
        }
    }
    return count;
}

fn second(input_file: &str) -> u32 {
    let input = read_input(input_file);
    let mut total = 0;
    for line in input.lines() {
        let digits = parse_digits(line.to_string());
        let map = guess_map(digits);
        let number = convert_digits(line.to_string(), map);
        total += number
    }
    return total;
}

fn convert_digits(line: String, map: [usize; 7]) -> u32 {
    let l: Vec<&str> = line.split(" | ").collect();
    let digits_output: Vec<&str> = l[1].split(" ").collect();
    let mut s = "".to_string();
    for digit in digits_output {
        let new_digit = digit_to_char(digit, map);
        s.push(new_digit);
    }
    return s.parse().unwrap();
}

fn digit_to_char(string: &str, map: [usize; 7]) -> char {
    let s: Vec<char> = string.chars().collect();
    let mut n = [false; 7];
    for c in s {
        match c {
            'a' => n[map[0]] = true,
            'b' => n[map[1]] = true,
            'c' => n[map[2]] = true,
            'd' => n[map[3]] = true,
            'e' => n[map[4]] = true,
            'f' => n[map[5]] = true,
            'g' => n[map[6]] = true,
            _ => {}
        }
    }
    match n {
        [true, true, true, false, true, true, true] => '0',
        [false, false, true, false, false, true, false] => '1',
        [true, false, true, true, true, false, true] => '2',
        [true, false, true, true, false, true, true] => '3',
        [false, true, true, true, false, true, false] => '4',
        [true, true, false, true, false, true, true] => '5',
        [true, true, false, true, true, true, true] => '6',
        [true, false, true, false, false, true, false] => '7',
        [true, true, true, true, true, true, true] => '8',
        [true, true, true, true, false, true, true] => '9',
        _ => 'e',
    }
}
fn guess_map(digits: HashMap<u8, Vec<[bool; 7]>>) -> [usize; 7] {
    let mut map = [7; 7];
    let a = guess_a(digits.clone());
    let f = guess_f(digits.clone());
    let c = guess_c(digits.clone(), f);
    let g = guess_g(digits.clone(), a);
    let d = guess_d(digits.clone(), a, g);
    let b = guess_b(digits.clone(), d);
    map[a] = 0;
    map[b] = 1;
    map[c] = 2;
    map[d] = 3;
    map[f] = 5;
    map[g] = 6;
    let e = map.iter().position(|r| r == &7).unwrap();
    map[e] = 4;
    return map;
}

fn guess_a(digits: HashMap<u8, Vec<[bool; 7]>>) -> usize {
    for i in 0..7 {
        if digits[&2][0][i].to_owned() != digits[&3][0][i].to_owned() {
            return i;
        }
    }
    return 0;
}

fn guess_f(digits: HashMap<u8, Vec<[bool; 7]>>) -> usize {
    let mut test = Vec::new();
    for i in [2, 3, 4, 6] {
        test.append(&mut digits[&i].to_owned());
    }
    let mut r = [true; 7];
    for t in test {
        r = all(r, t)
    }
    return r.iter().position(|r| r == &true).unwrap();
}

fn guess_c(digits: HashMap<u8, Vec<[bool; 7]>>, f: usize) -> usize {
    let mut test = Vec::new();
    for i in [2, 3, 4, 7] {
        test.append(&mut digits[&i].to_owned());
    }
    let mut r = [true; 7];
    r[f as usize] = false;
    for t in test {
        r = all(r, t)
    }
    return r.iter().position(|r| r == &true).unwrap();
}

fn guess_g(digits: HashMap<u8, Vec<[bool; 7]>>, a: usize) -> usize {
    let mut test = Vec::new();
    for i in [5, 6] {
        test.append(&mut digits[&i].to_owned());
    }
    let mut r = [true; 7];
    r[a as usize] = false;
    for t in test {
        r = all(r, t)
    }
    return r.iter().position(|r| r == &true).unwrap();
}

fn guess_d(digits: HashMap<u8, Vec<[bool; 7]>>, a: usize, g: usize) -> usize {
    let mut test = Vec::new();
    for i in [5] {
        test.append(&mut digits[&i].to_owned());
    }
    let mut r = [true; 7];
    r[a as usize] = false;
    r[g as usize] = false;
    for t in test {
        r = all(r, t)
    }
    return r.iter().position(|r| r == &true).unwrap();
}

fn guess_b(digits: HashMap<u8, Vec<[bool; 7]>>, d: usize) -> usize {
    for i in 0..7 {
        if i == d {
            continue;
        }
        if digits[&2][0][i].to_owned() != digits[&4][0][i].to_owned() {
            return i;
        }
    }
    return 0;
}

fn all(a: [bool; 7], b: [bool; 7]) -> [bool; 7] {
    let mut and = [false; 7];
    for i in 0..7 {
        and[i] = a[i] & b[i]
    }
    and
}

fn parse_digits(line: String) -> HashMap<u8, Vec<[bool; 7]>> {
    let mut digits: HashMap<u8, Vec<[bool; 7]>> = HashMap::new();
    for i in 2..8 {
        digits.insert(i, Vec::new());
    }
    let l: Vec<&str> = line.split(" | ").collect();
    let digits_input: Vec<&str> = l[0].split(" ").collect();
    for d in digits_input {
        let mut digit = [false; 7];
        for c in d.chars() {
            match c {
                'a' => digit[0] = true,
                'b' => digit[1] = true,
                'c' => digit[2] = true,
                'd' => digit[3] = true,
                'e' => digit[4] = true,
                'f' => digit[5] = true,
                'g' => digit[6] = true,
                _ => {}
            }
        }
        let len = d.len() as u8;
        let mut digits_array: Vec<[bool; 7]> = (&digits[&len]).to_owned();
        add_digit(&mut digits_array, digit);
        digits.insert(len, digits_array);
    }
    return digits;
}

fn add_digit(digits: &mut Vec<[bool; 7]>, digit: [bool; 7]) {
    for d in digits.clone() {
        if digit == d.to_owned() {
            return;
        }
    }
    digits.push(digit);
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
        assert_eq!(first("input_test"), 26);
    }

    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 61229);
    }

}
