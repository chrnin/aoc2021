use std::collections::HashMap;
use std::fs;

struct Dice {
    state: u64,
}

impl Dice {
    fn new() -> Dice {
        return Dice { state: 1 };
    }
    fn roll(&mut self) -> u64 {
        let result = self.state;
        self.state = (self.state % 100) + 1;
        return result;
    }
}

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(filename: &str) -> u64 {
    let (mut p1, mut p2) = read_input(filename);
    p1 -= 1;
    p2 -= 1;
    let (mut score_p1, mut score_p2) = (0, 0);
    let mut dice = Dice::new();

    let mut turn = 0;
    while score_p1 < 1000 && score_p2 < 1000 {
        if turn % 2 == 0 {
            p1 = (p1 + dice.roll() + dice.roll() + dice.roll()) % 10;
            score_p1 += p1 + 1;
        } else {
            p2 = (p2 + dice.roll() + dice.roll() + dice.roll()) % 10;
            score_p2 += p2 + 1;
        }
        turn += 1;
    }
    if score_p1 > score_p2 {
        return score_p2 * turn * 3;
    } else {
        return score_p1 * turn * 3;
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Player {
    state: u64,
    score: u64,
}
fn second(filename: &str) -> u64 {
    let mut states: HashMap<[Player; 2], u64> = HashMap::new();
    let (p1, p2) = read_input(filename);
    let init = [
        Player {
            state: p1,
            score: 0,
        },
        Player {
            state: p2,
            score: 0,
        },
    ];
    let _ = states.entry(init).or_insert(1);
    let mut finished = false;
    while !finished {
        finished = true;
        let mut new_states: HashMap<[Player; 2], u64> = HashMap::new();
        for ([player1, player2], n) in states.clone() {
            if player1.score < 21 && player2.score < 21 && n > 0 {
                finished = false;
                let result1 = throw_dirac_dices(&player1);
                for (n1, next_player1) in &result1 {
                    if next_player1.score >= 21 {
                        *new_states.entry([*next_player1, player2]).or_insert(0) += n1 * n;
                    }
                }
                let result2 = throw_dirac_dices(&player2);
                for [(n1, next_player1), (n2, next_player2)] in pairs(&result1, &result2) {
                    if next_player1.score < 21 {
                        *new_states.entry([next_player1, next_player2]).or_insert(0) += n1 * n2 * n;
                    } 
                }
            } else {
                *new_states.entry([player1, player2]).or_insert(0) += n;
            }
        }
        states = new_states;
    }
    let mut p1wins = 0;
    let mut p2wins = 0;
    for ([player1, player2], n) in states {
        if player1.score >= player2.score {
            p1wins += n
        } else {
            p2wins += n
        }
    }
    p1wins.max(p2wins)
}

fn pairs<T: Clone>(vec1: &Vec<T>, vec2: &Vec<T>) -> Vec<[T; 2]> {
    let mut result = Vec::new();
    for elem1 in vec1 {
        for elem2 in vec2 {
            result.push([elem1.clone(), elem2.clone()])
        }
    }
    result
}

fn throw_dirac_dices(player: &Player) -> Vec<(u64, Player)> {
    let results = vec![[3, 1], [4, 3], [5, 6], [6, 7], [7, 6], [8, 3], [9, 1]];
    results
        .iter()
        .map(|&[result, dup]| {
            (
                dup,
                Player {
                    score: player.score + (player.state - 1 + result) % 10 + 1,
                    state: (player.state - 1 + result) % 10 + 1,
                },
            )
        })
        .collect()
}

fn read_input(filename: &str) -> (u64, u64) {
    let content = fs::read_to_string(filename).expect("can't read");
    let mut lines = content.lines();
    let p1 = lines.next().unwrap()[28..].parse().unwrap();
    let p2 = lines.next().unwrap()[28..].parse().unwrap();
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first("input_test"), 739785);
    }
    
    #[test]
    fn test_2() {
        assert_eq!(second("input_test"), 444356092776315);
    }
}