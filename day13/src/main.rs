use std::fs;
use std::collections::HashMap;
fn main() {
    println!("first: {}", first("input"));
    println!("second: \n{}", second("input"));
}

fn first(input_file: &str) -> usize {
    let (map, folds) = parse_input(input_file);
    let new_map = fold(&map, folds[0]);
    return new_map.len();
}

fn second(input_file: &str) -> String {
    let (mut map, folds) = parse_input(input_file);
    for f in folds {
        map = fold(&map, f);
    }
    return display(&map);
}

fn display(map: &HashMap<[u32;2],u8>) -> String {
    let mut x = 0;
    let mut y = 0;
    for (coord, _) in map {
        if coord[0] > x {
            x = coord[0];
        }
        if coord[1] > y {
            y = coord[1];
        }
    }
    let mut disp: Vec<char> = Vec::new();
    for n in 0..=y {
        for m in 0..=x {
            if map.contains_key(&[m,n]) {
                disp.push('#');
            } else {
                disp.push('.');
            }
        }
        disp.push('\n');
    }
    disp.into_iter().collect()
}

fn fold(map: &HashMap<[u32;2],u8>, fold: [u32;2]) -> HashMap<[u32;2],u8> {
    if fold[0] == 0 {
        return fold_x(map, fold[1])
    } else {
        return fold_y(map, fold[1])
    }
}

fn fold_x(map: &HashMap<[u32;2],u8>, x: u32) -> HashMap<[u32;2],u8>{
    let mut new_map: HashMap<[u32;2],u8> = HashMap::new();
    for (coord, point) in map {
        let mut new_x = coord[0];
        if coord[0] >= x {
            new_x = 2*(x) - (coord[0]);
        }
        *new_map.entry([new_x, coord[1]]).or_insert(0) += point;
    }
    return new_map
}

fn fold_y(map: &HashMap<[u32;2],u8>, y: u32) -> HashMap<[u32;2],u8>{
    let mut new_map: HashMap<[u32;2],u8> = HashMap::new();
    for (coord, point) in map {
        let mut new_y = coord[1];
        if coord[1] >= y {
            new_y = 2*y - (coord[1]) ;
        }
        *new_map.entry([coord[0], new_y]).or_insert(0) += point;
    }
    return new_map
}

fn parse_input(filename: &str) -> (HashMap<[u32;2],u8>, Vec<[u32; 2]>) {
    let mut map: HashMap<[u32;2],u8> = HashMap::new();
    let mut folds: Vec<[u32; 2]> = Vec::new();
    for line in read_input(filename).lines() {
        if line.len() > 0 {
            if &line[0..1] == "f" {
                let fold = line[13..].parse().unwrap();
                if &line[11..12] == "x" {
                    folds.push([0, fold])
                } else {
                    folds.push([1, fold])
                }
            } else {
                let coord: Vec<u32> = line.split(",").map(|c| c.parse().unwrap()).collect();
                map.insert([coord[0],coord[1]], 1);
            }
        }
    }
    return (map, folds);
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
        assert_eq!(first("input_test"), 17);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), read_input("output_test").to_string());
    }
}
