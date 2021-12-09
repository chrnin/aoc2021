use std::fs;

fn main() {
    println!("{}", first("input"));
    println!("{}", second("input"));
}

fn first(input_file: &str) -> u32 {
    let input = read_input(input_file);
    let floor = Floor::new(input);
    let mut risk = 0;
    for i in 0..floor.heights.len() {
        if floor.is_lower(i) {
            risk += (floor.heights[i] + 1) as u32
        }
    }
    return risk;
}

fn second(input_file: &str) -> u32 {
    let input = read_input(input_file);
    let floor = Floor::new(input);
    let mut bottom = Vec::new();
    for i in 0..floor.heights.len() {
        if floor.is_lower(i) {
            bottom.push(i)
        }
    }
    let mut bassins: Vec<Vec<usize>> = bottom.iter().map(|&b| floor.bassin(b)).collect();
    bassins.sort_by(|a,b| b.len().cmp(&a.len()));
    bassins.dedup();
    if bassins.len() >= 3 {
        return (bassins[0].len() * bassins[1].len() * bassins[2].len()) as u32
    }
    return 0;
}

struct Floor {
    heights: Vec<u8>,
    dim: [usize; 2],
}

impl Floor {
    fn bassin(&self, pos: usize) -> Vec<usize> {
        let mut bassin = vec![];
        let mut new = vec![pos];
        while new.len() > 0 {
            let pos = new.pop().unwrap();
            bassin.push(pos);
            new.append(
                &mut self
                    .higher_neighbours(pos)
                    .iter()
                    .filter(|&p| !bassin.contains(p) && !new.contains(p))
                    .map(|&p| p)
                    .collect(),
            );
        }
        let mut b: Vec<usize> = bassin.iter().filter(|&&p| self.heights[p] != 9).map(|&p| p).collect();
        b.sort();
        return b
    }

    fn new(input: String) -> Floor {
        let mut heights = Vec::new();
        let x: usize = input.lines().next().unwrap().chars().count();
        let y: usize = input.lines().count();
        for line in input.lines() {
            heights.append(
                &mut line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect(),
            )
        }
        return Floor {
            heights: heights,
            dim: [x, y],
        };
    }

    fn is_lower(&self, pos: usize) -> bool {
        let current = self.heights[pos];
        let neighbours = self.neighbours(pos);
        return current
            < neighbours
                .iter()
                .map(|n| self.heights[n.to_owned()])
                .min()
                .unwrap()
                .to_owned();
    }

    fn higher_neighbours(&self, pos: usize) -> Vec<usize> {
        self.neighbours(pos)
            .iter()
            .filter(|&&p| self.heights[p] > self.heights[pos])
            .map(|&p| p)
            .collect()
    }
    fn neighbours(&self, pos: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        if pos % self.dim[0] > 0 {
            neighbours.push(pos - 1);
        }
        if pos >= self.dim[0] {
            neighbours.push(pos - self.dim[0]);
        }
        if pos < self.heights.len() - self.dim[0] {
            neighbours.push(pos + self.dim[0]);
        }
        if pos % self.dim[0] != self.dim[0] - 1 {
            neighbours.push(pos + 1);
        }
        neighbours
    }
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
        assert_eq!(first("input_test"), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 1134);
    }
}
