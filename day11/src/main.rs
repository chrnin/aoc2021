use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(input: &str) -> u32 {
    let input = read_input(input);
    let mut area = Area::new(input);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += area.increment()
    }
    flashes
}

fn second(input: &str) -> u32 {
    let input = read_input(input);
    let mut area = Area::new(input);
    let mut round = 0;
    loop {
        round += 1;
        if area.increment() as usize == area.values.len() {
            return round
        }
    }
}

#[derive(Debug)]
struct Area {
    values: Vec<u8>,
    dim: [usize; 2],
}

impl Area {
    fn increment(&mut self) -> u32 {
        for i in 0..self.values.len() {
            self.values[i] += 1;
        }
        let mut unstable = true;
        let mut flashes = 0;
        while unstable {
            unstable = false;
            for i in 0..self.values.len() {
                if self.values[i] > 9 {
                    self.values[i] = 0;
                    flashes += 1;
                    for n in self.neighbours(i) {
                        if self.values[n] > 0 {
                            self.values[n] += 1;
                            unstable = true;
                        }
                    }
                }
            }
        }
        flashes
    }

    fn new(input: String) -> Area {
        let mut values = Vec::new();
        let x: usize = input.lines().next().unwrap().chars().count();
        let y: usize = input.lines().count();
        for line in input.lines() {
            values.append(
                &mut line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect(),
            )
        }
        return Area {
            values: values,
            dim: [x, y],
        };
    }

    fn neighbours(&self, pos: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        if pos >= self.dim[0] && pos % self.dim[0] > 0 {
            neighbours.push(pos - self.dim[0] - 1); // up-left
        }
        if pos >= self.dim[0] {
            neighbours.push(pos - self.dim[0]); // up
        }
        if pos >= self.dim[0] && pos % self.dim[0] != self.dim[0] - 1 {
            neighbours.push(pos - self.dim[0] + 1); // up-right
        }
        if pos % self.dim[0] > 0 {
            neighbours.push(pos - 1); // left
        }
        if pos % self.dim[0] != self.dim[0] - 1 {
            neighbours.push(pos + 1); // right
        }
        if pos < self.values.len() - self.dim[0] && pos % self.dim[0] > 0 {
            neighbours.push(pos + self.dim[0] - 1); // down-left
        }
        if pos < self.values.len() - self.dim[0] {
            neighbours.push(pos + self.dim[0]); // down
        }
        if pos < self.values.len() - self.dim[0] && pos % self.dim[0] != self.dim[0] - 1 {
            neighbours.push(pos + self.dim[0] + 1); // down-right
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
        assert_eq!(first("input_test"), 1656);
    }
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 195);
    }
}
