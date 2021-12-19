use std::fs;

#[derive(Debug, Clone)]
enum Number {
    Single(u8),
    Pair(Box<[Number; 2]>),
}

impl Number {
    fn new(object: &serde_json::Value) -> Option<Number> {
        if object.is_array() {
            if object.as_array().unwrap().len() != 2 {
                return None;
            } else {
                let a = Number::new(&object.as_array().unwrap()[0]);
                let b = Number::new(&object.as_array().unwrap()[1]);
                if Option::is_some(&a) && Option::is_some(&b) {
                    let pair = Box::new([a.unwrap(), b.unwrap()]);
                    return Some(Number::Pair(pair));
                } else {
                    return None;
                }
            }
        } else if object.is_u64() {
            return Some(Number::Single(object.as_u64().unwrap() as u8));
        } else {
            return None;
        }
    }

    fn add(&mut self, number: Number) {
        *self = Number::Pair(Box::new([self.clone(), number]));
        self.reduce();
    }

    fn reduce(&mut self) {
        if let None = self.explode(0) {
            if self.split() {
                let _ = self.reduce();
            }
        } else {
            let _ = self.reduce();
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Single(s) => {
                if *s >= 10 {
                    *self = Number::Pair(Box::new([
                        Number::Single(*s / 2),
                        Number::Single(*s / 2 + *s % 2),
                    ]));
                    return true;
                }
                false
            }
            Number::Pair(pair) => {
                return pair[0].split() || pair[1].split();
            }
        }
    }
    fn explode(&mut self, level: u8) -> Option<[Number; 2]> {
        if level < 4 {
            match self {
                Number::Pair(pair) => {
                    let p0 = pair[0].explode(level + 1);
                    if p0.is_some() {
                        if let Number::Single(value) = p0.clone().unwrap()[1] {
                            self.add_right(value);
                            let mut new_p = p0.unwrap();
                            new_p[1] = Number::Single(0);
                            return Some(new_p);
                        }
                    }

                    let p1 = pair[1].explode(level + 1);
                    if p1.is_some() {
                        if let Number::Single(value) = p1.clone().unwrap()[0] {
                            self.add_left(value);
                            let mut new_p = p1.unwrap();
                            new_p[0] = Number::Single(0);
                            return Some(new_p);
                        }
                    }
                    return None;
                }
                Number::Single(_) => return None,
            }
        } else {
            match self {
                Number::Pair(a) => {
                    let value = Some(*a.clone());
                    *self = Number::Single(0);
                    return value;
                }
                Number::Single(_) => {
                    return None;
                }
            }
        }
    }
    fn add_right(&mut self, value: u8) {
        match self {
            Number::Pair(pair) => {
                pair[1].add_allleft(value);
            }
            Number::Single(a) => *a += value,
        }
    }
    fn add_allleft(&mut self, value: u8) {
        match self {
            Number::Pair(pair) => {
                pair[0].add_allleft(value);
            }
            Number::Single(a) => *a += value,
        }
    }
    fn add_left(&mut self, value: u8) {
        match self {
            Number::Pair(pair) => {
                pair[0].add_allright(value);
            }
            Number::Single(a) => *a += value,
        }
    }
    fn add_allright(&mut self, value: u8) {
        match self {
            Number::Pair(pair) => {
                pair[1].add_allright(value);
            }
            Number::Single(a) => *a += value,
        }
    }
    fn magnitude(&self) -> u64 {
        match self {
            Number::Pair(pair) => {
                return 3 * pair[0].clone().magnitude() + 2 * pair[1].clone().magnitude();
            }
            Number::Single(a) => return *a as u64,
        }
    }
}

fn first(filename: &str) -> u64 {
    let input = read_input(filename);
    let mut lines = input.lines();
    let line = lines.next().expect("empty file");
    let mut number = Number::new(&serde_json::from_str(line).unwrap()).expect("not a number !");
    loop {
        let line = lines.next();
        match line {
            Some(l) => {
                number.add(Number::new(&serde_json::from_str(l).unwrap()).expect("not a number !"));
            }
            None => break,
        }
    }
    number.magnitude()
}

fn second(filename: &str) -> u64 {
    let input = read_input(filename);
    let lines: Vec<&str> = input.lines().collect();

    let mut max = 0;
    for i in lines.iter() {
        for j in lines.iter() {
            if i != j {
                let m = Number::new(&serde_json::from_str(i).unwrap()).expect("not a number !");
                let mut n = Number::new(&serde_json::from_str(j).unwrap()).expect("not a number !");
                n.add(m);
                let magnitude = n.magnitude();
                if magnitude > max {
                    max = magnitude;
                }
            }
        }
    }
    return max;
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("can't read file")
}

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first("input_test"), 4140);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 3993);
    }
}
