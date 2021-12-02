use std::fs;

fn main() {
    let input = read_input("input");
    println!("1st: {}", first(input));

    let input = read_input("input");
    println!("2nd: {}", second(input));
}

fn first(input: String) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for line in input.lines() {
        let mv: Vec<&str> = line.split(" ").collect();
        let direction = mv[0];
        let distance: i32 = mv[1].parse().unwrap();
        match direction {
            "forward" => x += distance,
            "down" => y += distance,
            "up" => y -= distance,
            _ => println!("unknown pattern")
        }
        
        
    }
    return x*y
}

fn second(input: String) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    for line in input.lines() {
        let mv: Vec<&str> = line.split(" ").collect();
        let direction = mv[0];
        let distance: i32 = mv[1].parse().unwrap();
        match direction {
            "forward" => {
                x += distance;
                y += distance*aim;
            },
            "down" => {
                aim += distance;
            },
            "up" => {
                aim -= distance;
            },
            _ => println!("unknown pattern")
        }       
    }
    return x*y
}

fn read_input(file: &str) -> String {
    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_first() {
        let input = read_input("input_test");
        assert_eq!(first(input), 150);
    }

    #[test]
    fn test_second() {
        let input = read_input("input_test");
        assert_eq!(second(input), 900);
    }
}