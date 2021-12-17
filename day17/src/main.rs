use std::fs;

fn main() {
    println!("Hello, world!");
    let area = read_input("input");
    println!("first: {}", first(&area));
    println!("second: {}", second(&area));
}

fn first(area: &Area) -> i32 {
    sum_all(-area.bottom - 1)
}

fn second(area: &Area) -> i32 {
    let xs = check_x(area);
    let ys = check_y(area);
    let mut count: i32 = 0;
    for x in xs.iter() {
        for y in ys.iter() {
            if validate(area, x.clone(), y.clone()) {
                count += 1
            }
        }
    }
    count
}

fn validate(area: &Area, input_vel_x: i32, input_vel_y: i32) -> bool {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut vel_x = input_vel_x;
    let mut vel_y = input_vel_y;
    while pos_x <= area.right && pos_y >= area.bottom {
        pos_x += vel_x;
        pos_y += vel_y;
        if vel_x > 0 {
            vel_x -= 1
        }
        vel_y -= 1;
        if pos_x <= area.right && pos_y >= area.bottom && pos_x >= area.left && pos_y <= area.top {
            return true;
        }
    }
    return false;
}

fn check_x(area: &Area) -> Vec<i32> {
    let mut ok = Vec::new();
    for i in 0..area.right + 1 {
        if test_x(i, area.left, area.right) {
            ok.push(i)
        }
    }
    ok.sort();
    ok
}

fn check_y(area: &Area) -> Vec<i32> {
    let mut ok = Vec::new();
    for i in 0..-area.bottom + 1 {
        if test_y(i, -area.top, -area.bottom) {
            ok.push(-i);
            ok.push(i - 1);
        }
    }
    ok.sort();
    ok.dedup();
    ok
}

fn sum_all(v: i32) -> i32 {
    return v * (v + 1) / 2;
}

fn test_y(velocity: i32, min: i32, max: i32) -> bool {
    let mut vel_y = velocity;
    let mut pos = 0;
    while pos <= max {
        pos += vel_y;
        vel_y += 1;
        if pos >= min && pos <= max {
            return true;
        }
    }
    return false;
}
fn test_x(velocity: i32, min: i32, max: i32) -> bool {
    for i in 0..velocity + 1 {
        let pos = sum_all(velocity) - sum_all(velocity - i);
        if pos >= min && pos <= max {
            return true;
        }
    }
    false
}

#[derive(Debug)]
struct Area {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

fn read_input(filename: &str) -> Area {
    let content = fs::read_to_string(filename).expect("can't read file");
    let coords: Vec<i32> = content[13..content.len() - 1]
        .split(", ")
        .flat_map(|s| s[2..].split(".."))
        .map(|c| c.parse().unwrap())
        .collect();

    Area {
        left: coords[0],
        right: coords[1],
        bottom: coords[2],
        top: coords[3],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = read_input("input_test");
        assert_eq!(first(&input), 45);
    }
    #[test]
    fn test_2() {
        let input = read_input("input_test");
        assert_eq!(second(&input), 112);
    }
}
