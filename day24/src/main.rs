use std::fs;

fn main() {
    let (min, max) = day24("input");
    println!("first: {}", max);
    println!("second: {}", min);
}

fn day24(filename: &str) -> (i64, i64) {
    let values = read_input(filename);
    solve(&values)
}

fn solve(values: &Vec<[i64; 3]>) -> (i64, i64) {
    let mut max = vec![0; values.len() ];
    let mut min = vec![0; values.len() ];
    for [i, j] in pairs(values) {
        let diff = values[i][1] + values[j][2];
        if diff < 0 {
            max[j] = 9;
            max[i] = 9 + diff;
            min[j] = 1 - diff;
            min[i] = 1;
        } else {
            max[j] = 9 - diff;
            max[i] = 9;
            min[j] = 1;
            min[i] = 1 + diff;
        }
    }
    (
        min.iter().rev().enumerate().map(|(i, &v)| v * 10_i64.pow(i as u32)).sum(),
        max.iter().rev().enumerate().map(|(i, &v)| v * 10_i64.pow(i as u32)).sum(),
    )
}

fn pairs(values: &Vec<[i64; 3]>) -> Vec<[usize; 2]> {
    let mut stack = Vec::new();
    let mut pairs = Vec::new();
    for (i, value) in values.iter().enumerate() {
        if value[0] == 1 {
            stack.push(i)
        } else {
            pairs.push([i, stack.pop().unwrap()])
        }
    }
    pairs
}

fn read_input(filename: &str) -> Vec<[i64; 3]> {
    let content = fs::read_to_string(filename).expect("can't read");
    let mut values = Vec::new();
    let mut value = [0, 0, 0];
    for (i, line) in content.lines().enumerate() {
        if i % 18 == 4 {
            value[0] = line.split(' ').nth(2).unwrap().parse().unwrap();
        }
        if i % 18 == 5 {
            value[1] = line.split(' ').nth(2).unwrap().parse().unwrap();
        }
        if i % 18 == 15 {
            value[2] = line.split(' ').nth(2).unwrap().parse().unwrap();
        }
        if i % 18 == 17 {
            values.push(value);
        }
    }
    values
}
