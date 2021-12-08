use std::fs;

fn main() {
    let input = read_input();
    println!("1st: {}", first(input));

    let input = read_input();
    println!("2nd: {}", second(input));
}

fn first(input: String) -> i32 {
    let mut prec: i32 = 0;
    let mut count: i32 = -1;
    for l in input.lines() {
        let current: i32 = l.parse().unwrap();
        if current > prec {
            count += 1;
        }
        prec = current;
    }
    return count
}

fn second(input: String) -> i32 {
    // deuxième problème
    let mut prec: i32 = 0;
    let mut values: [i32; 3] = [0, 0, 0];
    let mut count: i32 = -3;

    for (i, l) in input.lines().enumerate() {
        values[i%3] = l.parse().unwrap();
        let current: i32 = values.iter().sum();
        if current > prec {
            count += 1
        }
        prec = current;
    }
    return count
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
        let input: String = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n".to_string();
        assert_eq!(first(input), 7);
    }

    #[test]
    fn test_second() {
        let input: String = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n".to_string();
        assert_eq!(second(input), 5);
    }
}