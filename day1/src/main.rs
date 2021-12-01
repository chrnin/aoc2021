use std::fs;

fn main() {
    let input = read_input();
    let mut prec: i32 = 0;
    let mut count: i32 = 0;

    // premier probleme
    for l in input.lines() {
        let current: i32 = l.parse().unwrap();
        if current > prec {
            count += 1;
        }
        prec = current;
    }
    println!("1st: {}", count - 1);

    // deuxième problème
    let mut prec: i32 = 0;
    let mut values: [i32; 3] = [0, 0, 0];
    let mut count: i32 = 0;

    for (i, l) in input.lines().enumerate() {
        values[i%3] = l.parse().unwrap();
        let current: i32 = values.iter().sum();
        if current > prec {
            count += 1
        }
        prec = current;
    }
    println!("2nd: {}", count-3);
}

fn read_input() -> String {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    return contents
}