use std::fs;

fn main() {
    let input = read_input("input");
    println!("1st: {}", first(input.clone()));
    println!("2nd: {}", second(input));
}


fn first(input: String) -> i32 {
    let (actions, mut tables) = parse_input(input);
    for action in actions {
        tables = action_on_tables(tables.clone(), action);
        for table in tables.clone() {
            let check = check_table(table.clone());
            if check {
                return sum(table) * action
            }
        }
    }
    return 0
}

fn second(input: String) -> i32 {
    let (actions, mut tables) = parse_input(input);
    let mut loser: usize = 0;
    for action in actions {
        let mut checks: i32 = 0;
        let mut i: usize = 0;
        tables = action_on_tables(tables.clone(), action);
        for table in tables.clone() {
            let check = check_table(table.clone());
            if !check {
                checks += 1;
                loser = i;
            }
            i += 1;
        }
        if checks == 0 {
            let loser_tables = tables.clone();
            let loser_table = &loser_tables[loser];
            return sum(loser_table.to_vec()) * action;
        }
    }
    return 0
}

fn sum(table: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in table {
        if i >= 0 {
            sum += i;
        }
    }
    return sum
}

fn action_on_tables(tables: Vec<Vec<i32>>, action: i32) -> Vec<Vec<i32>> {
    let mut new_tables = Vec::new();
    for mut table in tables {
        for (i, value) in table.clone().iter().enumerate() {
            if value == &action {
                table[i] = -1;
            }
        }
        new_tables.push(table)
    }

    return new_tables;
}


fn check_table(table: Vec<i32>) -> bool {
    for i in 0..5 {
        if table[0+i] + table[5+i] + table[10+i] + table[15+i] + table[20+i] == -5 ||
        table[0+i*5] + table[1+i*5] + table[2+i*5] + table[3+i*5] + table[4+i*5] == -5 {
            return true
        }
    }
    return false
}

fn parse_input(input: String) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut lines = input.lines();
    let actions: Vec<i32> = lines.next()
                              .unwrap()
                              .split(",")
                              .map(|i| i.parse().unwrap())
                              .collect();
    let _ = lines.next(); 
    
    let mut tables = Vec::new();
    let mut table: Vec<i32> = Vec::new();
    loop {
        match lines.next() {
            Some(line) => {
                if line != "" {
                    let mut t: Vec<i32> = line.replace("  ", " ")
                                              .trim()
                                              .split(" ")
                                              .map(|i| i.parse().unwrap())
                                              .collect();
                    table.append(&mut t);
                } else {
                    tables.push(table.clone());
                    table = Vec::new();
                    continue
                }
            },
            None => {
                tables.push(table.clone());
                break
            },
        }
    }
    return (actions, tables);

}

fn read_input(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_first() {
        let input = read_input("input_test");
        assert_eq!(first(input), 4512);
    }

    #[test]
    fn test_second() {
        let input = read_input("input_test");
        assert_eq!(second(input), 1924);
    }
}