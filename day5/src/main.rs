use std::fs;

fn main() {
    let input = read_input("input");
    println!("first: {}", first(input));
    let input = read_input("input");
    println!("second: {}", second(input))
}

fn first(input: String) -> i32 {
    let mut space = [0; 1000000];
    for line in input.lines() {
        let (x1, y1, x2, y2) = parse_coords(&line);
        if is_straight(x1, y1, x2, y2) {
            increment(&mut space, x1, y1, x2, y2);
        } 
    }
    return count_intersect(&mut space)
}

fn second(input: String) -> i32 {
    let mut space = [0; 1000000];
    for line in input.lines() {
        let (x1, y1, x2, y2) = parse_coords(&line);
        if is_straight(x1, y1, x2, y2) {
            increment(&mut space, x1, y1, x2, y2);
        } else {
            increment_oline(&mut space, x1, y1, x2, y2);
        }
    }
    return count_intersect(&mut space)
}

fn count_intersect(space: &mut [i32; 1000000]) -> i32 {
    let mut count = 0;
    for i in 0..1000000 {
        if space[i] > 1 {
            count += 1;
        }
    }
    return count
}

fn increment(space: &mut [i32; 1000000], x1: usize, y1: usize, x2: usize, y2: usize) {
    if x1 == x2 {
        increment_vline(space, y1, y2, x1);
        return;
    } 
    if y1 == y2 {
        increment_hline(space, x1, x2, y1);
        return;
    }
}

fn increment_hline(space: &mut [i32; 1000000], x1: usize, x2: usize, y: usize) {
    if x1 < x2 {
        for x in x1..x2+1 {
            space[1000*y+x] += 1;
        }
    } else {
        for x in x2..x1+1 {
            space[1000*y+x] += 1;
        }
    }
}

fn increment_oline(space: &mut [i32; 1000000], x1: usize, y1: usize, x2: usize, y2: usize) {
    let size: i32;
    let xstep: i32;
    let ystep: i32;
    // println!("writing {},{} to {},{}", x1, y1, x2, y2);
    match x1 < x2 {
        true => {
            size = (x2 - x1) as i32;
            xstep = 1;
        },
        false => {
            size = (x1 - x2) as i32;
            xstep = -1;
        },
    }
    match y1 < y2 {
        true => {
            ystep = 1;
        },
        false => {
            ystep = -1;
        },
    }
    for s in 0..size+1 {
        let x = (x1 as i32)+s*xstep;
        let y = (y1 as i32)+s*ystep;
        // println!("increment {} {}", x, y);
        space[(1000*y+x) as usize] += 1;
    }
}
fn increment_vline(space: &mut [i32; 1000000], y1: usize, y2: usize, x: usize) {
    if y1 < y2 {
        for y in y1..y2+1 {
            space[1000*y+x] += 1;
        }
    } else {
        for y in y2..y1+1 {
            space[1000*y+x] += 1;
        }
    }
}

fn is_straight(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    return x1 == x2 || y1 == y2 
}

fn parse_coords(line: &str) -> (usize, usize, usize, usize) {
    let coords: Vec<&str> = line.split(" -> ").collect();
    let c1: Vec<usize> = coords[0].split(",").map(|c| c.parse().unwrap()).collect();
    let c2: Vec<usize> = coords[1].split(",").map(|c| c.parse().unwrap()).collect();
    return (c1[0], c1[1], c2[0], c2[1])
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
        let input = read_input("input_test");
        assert_eq!(first(input), 5);
    }

    #[test]
    fn test_2() {
        let input = read_input("input_test");
        assert_eq!(second(input), 12);
    }
}