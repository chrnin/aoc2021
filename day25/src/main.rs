use std::fs;

fn main() {
    println!("first: {}", first("input"));
}

fn first(filename: &str) -> i32 {
    let mut floor = read_input(filename);
        
    let mut prec = floor.clone();
    step(&mut floor);

    
    let mut count = 1;
    while prec != floor {
        prec = floor.clone();
        step(&mut floor);
        count += 1;
    }

    count
    
}

fn step(floor: &mut Vec<Vec<char>>) {
    for y in 0..floor.len(){
        for x in 0..floor[0].len() {
            let n = (x+1)%floor[0].len();
            if floor[y][n] == '.' && floor[y][x] == '>' {
                floor[y][x] = '-';
                floor[y][n] = '<';
            }
        }
    }

    for y in 0..floor.len() {
        let n = (y+1)%floor.len();
        for x in 0..floor[0].len() {
            if (floor[n][x] == '.' || floor[n][x] == '-') && floor[y][x] == 'v' {
                floor[y][x] = '|';
                floor[n][x] = '^';
            }
        }
    }
    for y in 0..floor.len() {
        for x in 0..floor[0].len() {
            if floor[y][x] == '-' || floor[y][x] == '|' {
                floor[y][x] = '.'
            }
            if floor[y][x] == '^' {
                floor[y][x] = 'v'
            }
            if floor[y][x] == '<' {
                floor[y][x] = '>'
            }
        }
    }
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(filename).unwrap();
    content.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first("input_test"), 58);
    }
}