use std::collections::{HashMap, HashSet};
use std::fs;

const ROTATIONS: [[[i16; 3]; 3]; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
];

fn main() {
    let (first, second) = day19("input");
    println!("first: {}\nsecond: {}", first, second)
}

fn day19(filename: &str) -> (usize, i16) {
    let scanners = read_input(filename);

    let distances = scanners_with_distance(&scanners);
    let similarities = scanners_similarity(&distances);
    let path = compute_path(&similarities, scanners.len());
    let mut transformations = HashMap::new();
    transformations.insert(0, ([0, 0, 0], 0));

    for (a, b, similarity) in path.iter() {
        let (translation_a, rotation_a) = transformations.get(a).unwrap();

        let points_a: Vec<[i16; 3]> = transform(
            similarity
                .iter()
                .flat_map(|distance| distances[*a][distance])
                .collect(),
            *translation_a,
            rotation_a,
        );
        let points_b: Vec<[i16; 3]> = similarity
            .iter()
            .flat_map(|distance| distances[*b][distance])
            .collect();
        let transformation = find_transformation(points_a, points_b);
        if transformation.is_some() {
            let _ = transformations.insert(*b, transformation.unwrap());
        } else {
            println!("{} {}: nok", a, b)
        }
    }

    let mut all_beacons: HashSet<[i16; 3]> = HashSet::new();
    for (i, beacons) in scanners.iter().enumerate() {
        let (translation, rotation) = transformations[&i];
        let new_beacons: HashSet<[i16; 3]> = transform(beacons.clone(), translation, &rotation)
            .iter()
            .map(|&b| b)
            .collect();
        all_beacons.extend(&new_beacons)
    }

    let translations: Vec<[i16;3]> = transformations.values().map(|&(a,_)| a.clone()).collect();
    let translations_pairs = pairs(&translations);

    let manhattan_distances: Vec<i16> = translations_pairs.iter().map(|&(a, _, _)| manhattan_distance(a[0], a[1])).collect();
    (all_beacons.iter().count(), *manhattan_distances.iter().max().unwrap())
}

fn manhattan_distance(a: [i16;3], b:[i16;3]) -> i16 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

fn find_transformation(a: Vec<[i16; 3]>, b: Vec<[i16; 3]>) -> Option<([i16; 3], usize)> {
    for point_a in a.clone() {
        for point_b in b.clone() {
            for (rotation, _) in ROTATIONS.iter().enumerate() {
                let translation = get_translation(point_a, rotate(point_b, &rotation));
                let new_b: HashSet<[i16; 3]> = transform(b.clone(), translation, &rotation)
                    .iter()
                    .map(|&p| p)
                    .collect();
                if new_b.intersection(&a.iter().map(|&b| b).collect()).count() > 10 {
                    return Some((translation, rotation));
                }
            }
        }
    }
    return None;
}

fn get_translation(a: [i16; 3], b: [i16; 3]) -> [i16; 3] {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

fn transform(beacons: Vec<[i16; 3]>, translation: [i16; 3], rotation: &usize) -> Vec<[i16; 3]> {
    beacons
        .iter()
        .map(|&beacon| shift(rotate(beacon, rotation), translation))
        .collect()
}

fn shift(beacon: [i16; 3], translation: [i16; 3]) -> [i16; 3] {
    return [
        beacon[0] + translation[0],
        beacon[1] + translation[1],
        beacon[2] + translation[2],
    ];
}

fn rotate(point: [i16; 3], rotation: &usize) -> [i16; 3] {
    let mut new_point: [i16; 3] = [0, 0, 0];
    for i in 0..3 {
        new_point[i] = point[0] * ROTATIONS[*rotation][i][0]
            + point[1] * ROTATIONS[*rotation][i][1]
            + point[2] * ROTATIONS[*rotation][i][2]
    }
    new_point
}

fn compute_path(
    similarities: &Vec<(usize, usize, Vec<i32>)>,
    len: usize,
) -> Vec<(usize, usize, Vec<i32>)> {
    let mut path = vec![0];
    let mut path_with_origin = vec![];
    let mut count = 0;
    while path.len() < len {
        for (a, b, similarity) in similarities {
            if path.contains(&a) && !path.contains(&b) {
                path.push(*b);
                path_with_origin.push((*a, *b, similarity.clone()));
            }
            if path.contains(&b) && !path.contains(&a) {
                path.push(*a);
                path_with_origin.push((*b, *a, similarity.clone()));
            }
        }
        if count > len {
            panic!("Les informations ne permettent pas de résoudre ce problème")
        }
        count += 1
    }
    return path_with_origin;
}

fn scanners_similarity(
    scanners_with_distance: &Vec<HashMap<i32, [[i16; 3]; 2]>>,
) -> Vec<(usize, usize, Vec<i32>)> {
    let mut similarities = Vec::new();
    for ([a, b], i, j) in pairs(scanners_with_distance) {
        let akeys: HashSet<&i32> = a.keys().collect();
        let bkeys: HashSet<&i32> = b.keys().collect();
        let intersection = akeys.intersection(&bkeys);
        if intersection.clone().count() > 40 {
            similarities.push((i, j, intersection.map(|&&distance| distance).collect()));
        }
    }
    similarities
}

fn scanners_with_distance(scanners: &Vec<Vec<[i16; 3]>>) -> Vec<HashMap<i32, [[i16; 3]; 2]>> {
    let mut s = Vec::new();
    for scanner in scanners {
        let d = pairs_distance(
            pairs(scanner)
                .iter()
                .map(|&p| {
                    let (pair, _, _) = p;
                    return pair;
                })
                .collect(),
        );
        s.push(d);
    }
    s
}

fn pairs_distance(pairs: Vec<[[i16; 3]; 2]>) -> HashMap<i32, [[i16; 3]; 2]> {
    let mut distances = HashMap::new();
    pairs.iter().for_each(|&p| {
        let _ = distances.insert(distance(p[0], p[1]), p);
    });
    return distances;
}

fn distance(a: [i16; 3], b: [i16; 3]) -> i32 {
    ((a[0] - b[0]) as i32).pow(2) + ((a[1] - b[1]) as i32).pow(2) + ((a[2] - b[2]) as i32).pow(2)
}

fn pairs<T: Clone>(vec: &Vec<T>) -> Vec<([T; 2], usize, usize)> {
    let mut pairs = Vec::new();
    for (i, el1) in vec.iter().enumerate() {
        for (j, el2) in vec[i + 1..].iter().enumerate() {
            pairs.push(([el1.clone(), el2.clone()], i, i + j + 1));
        }
    }
    pairs
}

fn read_input(filename: &str) -> Vec<Vec<[i16; 3]>> {
    let content = fs::read_to_string(filename).expect("lecture impossible");
    let mut input = Vec::new();
    for scan in content.split("--- scan") {
        let mut scanner = Vec::new();
        for line in scan.lines() {
            if line.len() > 0 && &line[0..1] != "n" {
                let beacon: Vec<i16> = line.split(",").map(|c| c.parse().unwrap()).collect();
                scanner.push([beacon[0], beacon[1], beacon[2]]);
            }
        }
        if scanner.len() > 0 {
            input.push(scanner);
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(day19("input_test"), (79, 3621));
    }
    
}