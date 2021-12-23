use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

#[derive(Debug, Clone)]
struct Instruction {
    on: bool,
    area: [[i64; 2]; 3],
}

fn first(filename: &str) -> i64 {
    let instructions = &read_input(filename);
    let mut xs = vec![-50, 50];
    let mut ys = vec![-50, 50];
    let mut zs = vec![-50, 50];
    for instruction in instructions {
        xs.push(instruction.area[0][0]);
        xs.push(instruction.area[0][1]);
        ys.push(instruction.area[1][0]);
        ys.push(instruction.area[1][1]);
        zs.push(instruction.area[2][0]);
        zs.push(instruction.area[2][1]);
    }
    xs.sort();
    xs.dedup();
    xs = xs
        .iter()
        .filter(|&&x| x >= -50 && x <= 51)
        .map(|&x| x)
        .collect();
    ys.sort();
    ys.dedup();
    ys = ys
        .iter()
        .filter(|&&x| x >= -50 && x <= 51)
        .map(|&x| x)
        .collect();
    zs.sort();
    zs.dedup();
    zs = zs
        .iter()
        .filter(|&&x| x >= -50 && x <= 51)
        .map(|&x| x)
        .collect();

    let mut volume = 0;
    for x in lags(&xs) {
        for y in lags(&ys) {
            for z in lags(&zs) {
                if let Some(i) = instructions
                    .iter()
                    .rev()
                    .filter(|&i| inside(i.area, [x[0], y[0], z[0]]))
                    .next()
                {
                    if i.on {
                        volume += (x[1] - x[0]) * (y[1] - y[0]) * (z[1] - z[0]);
                    }
                }
            }
        }
    }
    volume
}

fn second(filename: &str) -> i64 {
    let instructions = &read_input(filename);
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();
    for instruction in instructions {
        xs.push(instruction.area[0][0]);
        xs.push(instruction.area[0][1]);
        ys.push(instruction.area[1][0]);
        ys.push(instruction.area[1][1]);
        zs.push(instruction.area[2][0]);
        zs.push(instruction.area[2][1]);
    }
    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();
    zs.sort();
    zs.dedup();

    let mut volume = 0;
    for x in lags(&xs) {
        let instructions_x: Vec<&Instruction> = instructions
            .iter()
            .rev()
            .filter(|&i| x[0] >= i.area[0][0] && x[0] < i.area[0][1])
            .collect();
        for y in lags(&ys) {
            let instructions_y: Vec<&Instruction> = instructions_x
                .iter()
                .rev()
                .filter(|&i| y[0] >= i.area[1][0] && y[0] < i.area[1][1]).map(|&i| i)
                .collect();
            for z in lags(&zs) {
                if let Some(i) = instructions_y
                    .iter()
                    .rev()
                    .filter(|&i| inside(i.area, [x[0], y[0], z[0]]))
                    .next()
                {
                    if i.on {
                        volume += (x[1] - x[0]) * (y[1] - y[0]) * (z[1] - z[0]);
                    }
                }
            }
        }
    }
    volume
}

fn inside(b: [[i64; 2]; 3], a: [i64; 3]) -> bool {
    a[0] >= b[0][0]
        && a[0] < b[0][1]
        && a[1] >= b[1][0]
        && a[1] < b[1][1]
        && a[2] >= b[2][0]
        && a[2] < b[2][1]
}

fn lags<T: Clone>(vec: &Vec<T>) -> Vec<[T; 2]> {
    let mut lags = Vec::new();
    for i in 0..vec.len() - 1 {
        lags.push([vec[i].clone(), vec[i + 1].clone()])
    }
    lags
}

fn read_input(filename: &str) -> Vec<Instruction> {
    let content = fs::read_to_string(filename).expect("can't read");
    content
        .lines()
        .map(|line| {
            let switch: Vec<&str> = line.split(" ").collect();
            let on: bool = switch[0] == "on";
            let mut area: [[i64; 2]; 3] = [[0, 0], [0, 0], [0, 0]];
            for (i, v) in switch[1].split(",").enumerate() {
                let mut coords = v[2..].split("..");
                area[i][0] = coords.next().unwrap().parse().unwrap();
                area[i][1] = coords.next().unwrap().parse::<i64>().unwrap() + 1;
            }
            Instruction { on: on, area: area }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(first("input_test"), 590784);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test2"), 2758514936282235)
    }
}
