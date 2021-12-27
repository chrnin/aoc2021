use std::collections::HashSet;
use std::fs;

fn main() {
    println!("{}", first("input"));
    println!("{}", second("input"));
}


fn first(filename: &str) -> i64 {
    let (algo, floor) = read_input(filename);
    let floor = floor.step(&algo);
    let floor = floor.step(&algo);
    floor.count()
}

fn second(filename: &str) -> i64 {
    let (algo, mut floor) = read_input(filename);
    for _ in 0..50 {
        floor = floor.step(&algo);
    }
    floor.count()
}
struct Floor {
    floor: Vec<Vec<bool>>,
    edge: bool,
}

impl Floor {
    fn count(&self) -> i64 {
        self.floor
            .iter()
            .flat_map(|line| line.iter().map(|&c| if c { 1 } else { 0 }))
            .sum()
    }
    fn step(&self, algo: &HashSet<usize>) -> Floor {
        let mut floor = Vec::new();
        for y in -1..(self.floor.len() + 1) as i64 {
            let mut line = Vec::new();
            for x in -1..(self.floor.len() + 1) as i64 {
                line.push(self.next(algo, x, y));
            }
            floor.push(line);
        }
        let edge = self.next(algo, -2, -2);
        Floor { floor, edge }
    }

    fn next(&self, algo: &HashSet<usize>, x: i64, y: i64) -> bool {
        algo.contains(&self.read(x, y))
    }

    fn read(&self, x: i64, y: i64) -> usize {
        let position = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        Floor::number(position.iter().map(|&(x, y)| self.get(x, y)).collect())
    }

    fn get(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 || y >= self.floor.len() as i64 || x >= self.floor[0].len() as i64 {
            return self.edge;
        } else {
            return self.floor[y as usize][x as usize];
        }
    }

    fn number(data: Vec<bool>) -> usize {
        data.iter()
            .rev()
            .enumerate()
            .map(|(i, &v)| if v { 2_usize.pow(i as u32) } else { 0 })
            .sum()
    }
}

fn read_input(filename: &str) -> (HashSet<usize>, Floor) {
    let content = fs::read_to_string(filename).expect("can't read");
    let mut lines = content.lines();
    let mut algo = HashSet::new();
    lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            if c == '#' {
                algo.insert(i);
            };
        });
    lines.next();
    let mut floor = Vec::new();
    while let Some(line) = lines.next() {
        floor.push(line.chars().map(|c| c == '#').collect())
    }
    (
        algo,
        Floor {
            floor: floor,
            edge: false,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first("input_test"), 35);
    }
    
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 3351);
    }
}