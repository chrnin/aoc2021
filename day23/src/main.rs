use pathfinding::prelude::dijkstra;
use std::fs;

fn main() {
    println!("first: {:?}", first("input"));
    println!("second: {:?}", second("input"));
}

fn first(filename: &str) -> i32 {
    let start = Hall::new(filename);
    let goal = Hall {
        way: [' '; 11],
        side: [
            vec!['A', 'A'],
            vec!['B', 'B'],
            vec!['C', 'C'],
            vec!['D', 'D'],
        ],
    };
    let result = dijkstra(&start, |p| p.neighbours(), |p| *p == goal);
    let (_, cost) = result.unwrap();
    return cost;
}

fn second(filename: &str) -> i32 {
    let start = Hall::new_second(filename);
    let goal = Hall {
        way: [' '; 11],
        side: [['A'; 4].to_vec(), ['B'; 4].to_vec(), ['C'; 4].to_vec(), ['D'; 4].to_vec()],
    };
    let result = dijkstra(&start, |p| p.neighbours(), |p| *p == goal);
    let (_, cost) = result.unwrap();
    return cost;
}

fn cost(c: char) -> i32 {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0,
    }
}

const SIDES: [char; 4] = ['A', 'B', 'C', 'D'];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hall {
    way: [char; 11],
    side: [Vec<char>; 4],
}

impl Hall {
    fn new(filename: &str) -> Hall {
        let content = fs::read_to_string(filename).expect("can't read");
        let mut lines = content.lines();
        let line1: Vec<char> = lines.nth(2).unwrap().chars().collect();
        let line2: Vec<char> = lines.next().unwrap().chars().collect();
        return Hall {
            way: [' '; 11],
            side: [
                vec![line1[3], line2[3]],
                vec![line1[5], line2[5]],
                vec![line1[7], line2[7]],
                vec![line1[9], line2[9]],
            ],
        };
    }

    fn new_second(filename: &str) -> Hall {
        let content = fs::read_to_string(filename).expect("can't read");
        let mut lines = content.lines();
        let line1: Vec<char> = lines.nth(2).unwrap().chars().collect();
        let line2: Vec<char> = lines.next().unwrap().chars().collect();
        return Hall {
            way: [' '; 11],
            side: [
                vec!(line1[3], 'D', 'D', line2[3]),
                vec!(line1[5], 'C', 'B', line2[5]),
                vec!(line1[7], 'B', 'A', line2[7]),
                vec!(line1[9], 'A', 'C', line2[9]),
            ],
        };
    }

    fn free_way(&self, i: usize) -> Vec<(usize, usize)> {
        let available = [0, 1, 3, 5, 7, 9, 10];
        let start: Vec<&usize> = available.iter().filter(|&p| p < &i).rev().collect();
        let end: Vec<&usize> = available.iter().filter(|&p| p > &i).collect();
        let mut possible = Vec::new();
        for &j in start {
            if self.way[j] == ' ' {
                possible.push((j, i - j))
            } else {
                break;
            }
        }
        for &j in end {
            if self.way[j] == ' ' {
                possible.push((j, j - i))
            } else {
                break;
            }
        }
        possible
    }

    fn free_side(&self, i: usize, c: char) -> Option<(usize, usize)> {
        let s = SIDES.iter().position(|&sc| sc == c).unwrap();
        let side_way = s * 2 + 2;
        for p in i.min(side_way) + 1..i.max(side_way) {
            if self.way[p] != ' ' {
                return None;
            }
        }
        if self.side[s]
            .iter()
            .filter(|&&sc| sc != ' ' && sc != c)
            .count()
            == 0
        {
            let distance = i.max(side_way) - i.min(side_way);
            if let Some(depth) = self.side[s].iter().position(|&c| c != ' ') {
                return Some((depth - 1, distance + depth));
            } else {
                return Some((self.side[s].len()-1, distance + self.side[s].len()));
            }
        }
        None
    }
    fn neighbours(&self) -> Vec<(Hall, i32)> {
        let mut neighbours = Vec::new();
        for (i, s) in SIDES.iter().enumerate() {
            if self.side[i].iter().filter(|&c| c != s && c != &' ').count() > 0 {
                let position = i * 2 + 2;
                let depth = self.side[i].iter().position(|p| p != &' ').unwrap();
                for (j, distance) in self.free_way(position) {
                    let mut hall = self.clone();
                    let c = hall.side[i][depth];
                    hall.way[j] = c;
                    hall.side[i][depth] = ' ';
                    let invoice = (distance + depth + 1) as i32 * cost(c);
                    neighbours.push((hall, invoice))
                }
                if let Some((destination_depth, distance)) =
                    self.free_side(position, self.side[i][depth])
                {
                    let mut hall = self.clone();
                    let destination = SIDES
                        .iter()
                        .position(|&p| p == self.side[i][depth])
                        .unwrap();
                    hall.side[i][depth] = ' ';
                    hall.side[destination][destination_depth] = self.side[i][depth];
                    let invoice = (depth + distance + 1) as i32 * cost(self.side[i][depth]);
                    neighbours.push((hall, invoice))
                }
            }
        }
        for i in [0, 1, 3, 5, 7, 9, 10] {
            if self.way[i] != ' ' {
                if let Some((depth, distance)) = self.free_side(i, self.way[i]) {
                    let mut hall = self.clone();
                    let c = hall.way[i];
                    let s = SIDES.iter().position(|&s| s == c).unwrap();
                    hall.way[i] = ' ';
                    hall.side[s][depth] = c;
                    let invoice = distance as i32 * cost(c);
                    neighbours.push((hall, invoice))
                }
            }
        }
        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first("input_test"), 12521);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 44169);
    }
}