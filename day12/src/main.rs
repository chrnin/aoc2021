use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(input_file: &str) -> usize {
    let map = parse_input(input_file);
    let pathes = explore("start".to_string(), map, vec![]);
    return pathes.len();
}

fn second(input_file: &str) -> usize {
    let map = parse_input(input_file);
    let pathes = explore_twice("start".to_string(), map, vec![]);
    return pathes.len();
}

fn filter(path: Vec<String>) -> bool {
    let mut mem: Vec<String> = Vec::new();
    let mut first = false;
    for cave in path {
        if cave.to_lowercase() == cave {
            if mem.contains(&cave) {
                if first {
                    return false;
                }
                first = true;
            }
            mem.push(cave)
        }
    }
    true
}
fn explore(position: String, map: Vec<[String; 2]>, path: Vec<String>) -> Vec<Vec<String>> {
    let mut pathes: Vec<Vec<String>> = Vec::new();
    for caves in map.clone() {
        if position.clone() == caves[0] || position.clone() == caves[1] {
            let destination: String;
            if position == caves[0] {
                destination = caves[1].clone();
            } else {
                destination = caves[0].clone();
            }
            if destination == "end" {
                let mut new_path = path.clone();
                new_path.push(position.clone());
                new_path.push("end".to_string());
                pathes.push(new_path.clone());
            } else {
                let new_map = filter_map(map.clone(), position.clone());
                let mut new_path = path.clone();
                new_path.push(position.clone());
                let mut next_path = new_path.clone();
                next_path.push(destination.clone());
                if filter(next_path.clone()) {
                    let mut new_pathes = explore(destination, new_map, new_path);
                    pathes.append(&mut new_pathes);
                }

            }
        }
    }
    return pathes;
}

fn explore_twice(
    position: String,
    map: Vec<[String; 2]>,
    path: Vec<String>,
) -> Vec<Vec<String>> {
    let mut pathes: Vec<Vec<String>> = Vec::new();
    for caves in map.clone() {
        if position.clone() == caves[0] || position.clone() == caves[1] {
            let destination: String;
            if position == caves[0] {
                destination = caves[1].clone();
            } else {
                destination = caves[0].clone();
            }
            if destination == "end" {
                let mut new_path = path.clone();
                new_path.push(position.clone());
                new_path.push("end".to_string());
                pathes.push(new_path.clone());
            } else {
                let mut new_map = map.clone();
                if path.contains(&position) || position == "start" {
                    new_map = filter_map(map.clone(), position.clone());
                }
                let mut new_path = path.clone();
                new_path.push(position.clone());
                let mut next_path = new_path.clone();
                next_path.push(destination.clone());
                if filter(next_path.clone()) {
                    let mut new_pathes = explore_twice(destination, new_map, new_path);
                    pathes.append(&mut new_pathes);
                }
            }
        }
    }
    return pathes;
}

fn filter_map(map: Vec<[String; 2]>, position: String) -> Vec<[String; 2]> {
    let mut new_map: Vec<[String; 2]> = Vec::new();
    if position.to_uppercase() == position {
        return map;
    }
    for m in map {
        if position != m[0] && position != m[1] {
            new_map.push(m)
        }
    }
    new_map
}

fn parse_input(filename: &str) -> Vec<[String; 2]> {
    let input = read_input(filename);
    input
        .lines()
        .map(|line| {
            let mut l = line.split("-");
            return [l.next().unwrap().to_owned(), l.next().unwrap().to_owned()];
        })
        .collect()
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
        assert_eq!(first("input_test"), 226);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 3509);
    }
}
