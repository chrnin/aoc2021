use pathfinding::prelude::dijkstra;
use std::fs;

fn first<'a>(filename: &'static str) -> usize {
    let map: Map = parse_input(filename);
    let goal: Pos = Pos::new(map.dim[0]-1, map.dim[0]-1);
    let result = dijkstra(&Pos::new(0, 0), |p| p.neighbours(&map), |p| *p == goal);
    let res = result.unwrap();
    res.1
}

fn second(filename: &'static str) -> usize {
    let map: Map = parse_input2(filename);
    let goal: Pos = Pos::new(map.dim[0]-1, map.dim[0]-1);
    let result = dijkstra(&Pos::new(0, 0), |p| p.neighbours(&map), |p| *p == goal);
    let res = result.unwrap();
    res.1
}

fn parse_input(filename: &'static str) -> Map {
    let input = read_input(filename);
    let mut heights = Vec::new();
    let x: i32 = input.lines().next().unwrap().chars().count() as i32;
    let y: i32 = input.lines().count() as i32;
    for line in input.lines() {
        heights.append(
            &mut line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        )
    }
    return Map {
        map: heights,
        dim: [x, y],
    };
}

fn parse_input2(filename: &'static str) -> Map {
    let input = read_input(filename);
    let mut heights = Vec::new();
    let x: i32 = (input.lines().next().unwrap().chars().count() * 5) as i32;
    let y: i32 = (input.lines().count() * 5) as i32;
    for line in input.lines() {
        for i in 0..5 {
        heights.append(
            &mut line
                .chars()
                .map(|c| ((c.to_digit(10).unwrap() -1 + i) % 9 + 1 ) as i32)
                .collect(),
        )
    }}
    let h = heights.clone();
    for i in 1..5 {
        heights.append(
            &mut h.iter()
                .map(|c| ((c - 1 + i) % 9 + 1) as i32)
                .collect()
        )
    }
    return Map {
        map: heights,
        dim: [x, y],
    };
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Map {
    dim: [i32; 2],
    map: Vec<i32>,
}

impl Map {
    fn cost(&self, pos: &Pos) -> usize {
        if pos.x <= self.dim[0] && pos.y <= self.dim[1] {
            return self.map[pos.index(self)] as usize;
        } else {
            return i16::MAX as usize;
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        return Pos {
            x: x,
            y: y,
        };
    }

    fn neighbours(&self, map: &Map) -> Vec<(Pos, usize)> {
        let mut n = Vec::new();
        if self.x < map.dim[0] - 1 { //right
            n.push(Pos::new(self.x + 1, self.y));
        }
        if self.x > 0 { // left
            n.push(Pos::new(self.x - 1, self.y));
        }
        if self.y < map.dim[1] - 1 { // down
            n.push(Pos::new(self.x, self.y + 1));
        }
        if self.y > 0 { // up
            n.push(Pos::new(self.x, self.y - 1));
        }
        n.into_iter().map(|p| (p.clone(), map.cost(&p))).collect()
    }
    fn index(&self, map: &Map) -> usize {
        return (self.x + self.y * map.dim[0]) as usize;
    }
}

fn read_input<'a>(filename: &'static str) -> String {
    let content = fs::read_to_string(filename).expect("lecture du fichier impossible");
    return content;
}

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first("input_test"), 40);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 316);
    }
}
