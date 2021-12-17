use std::fs;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    start: usize,
    end: usize,
    value: Option<u64>,
    length_type_id: Option<u8>,
    length: Option<u8>,
    subpackets: Vec<Packet>,
}

fn main() {
    let input = read_input("input");
    println!("first: {:?}", first(&input));
    println!("second: {:?}", second(&input));

}


fn first(input: &Vec<u8>) -> u32 {
    let packet = parse(input, 0);
    return sum_version(&packet);
}

fn second(input: &Vec<u8>) -> u64 {
    let packet = parse(input, 0);
    return eval(&packet);
}

fn sum_version(packet: &Packet) -> u32 {
    packet.version as u32 + packet.subpackets.iter().map(|p| sum_version(p) as u32).sum::<u32>() as u32
}

fn eval(packet: &Packet) -> u64 {
    match packet.type_id {
        0 => packet.subpackets.iter().map(|p| eval(p)).sum(),
        1 => packet.subpackets.iter().map(|p| eval(p)).product(),
        2 => packet.subpackets.iter().map(|p| eval(p)).min().unwrap(),
        3 => packet.subpackets.iter().map(|p| eval(p)).max().unwrap(),
        4 => packet.value.unwrap(),
        5 if eval(&packet.subpackets[0]) > eval(&packet.subpackets[1]) => 1,
        5 if eval(&packet.subpackets[0]) < eval(&packet.subpackets[1]) => 0,
        6 if eval(&packet.subpackets[0]) < eval(&packet.subpackets[1]) => 1,
        6 if eval(&packet.subpackets[0]) > eval(&packet.subpackets[1]) => 0,
        7 if eval(&packet.subpackets[0]) == eval(&packet.subpackets[1]) => 1,
        7 if eval(&packet.subpackets[0]) != eval(&packet.subpackets[1]) => 0,
        _ => 0,
    }
}
fn parse(input: &Vec<u8>, start: usize) -> Packet {
    let type_id = to_u32(&input[start + 3..start + 6].to_vec()) as u8;
    if type_id == 4 {
        parse_litteral(input, start)
    } else {
        parse_operator(input, start)
    }
}


fn parse_operator(input: &Vec<u8>, start: usize) -> Packet {
    let length_type_id = input[start + 6];
    let mut length: usize;
    let number: usize;
    let mut subpackets = Vec::new();
    if length_type_id == 0 {
        length = to_u32(&input[start + 7..start + 22].to_vec()) as usize;
        number = length.clone();
        let mut position = start + 22;
        while position < start + length + 16 {
            let packet = parse(input, position);
            position = packet.end.clone();
            subpackets.push(packet);
        }
        length = position-start;
    } else {
        number = to_u32(&input[start + 7..start + 18].to_vec()) as usize;
        let mut position = start + 18;
        for _ in 0..number {
            let packet = parse(input, position);
            position = packet.end.clone();
            subpackets.push(packet);
        }
        length = position-start;
    }

    return Packet {
        version: to_u32(&input[start..start + 3].to_vec()) as u8,
        type_id: to_u32(&input[start + 3..start + 6].to_vec()) as u8,
        start: start,
        end: start + length,
        value: None,
        length_type_id: Some(length_type_id),
        length: Some(number as u8),
        subpackets: subpackets,
    };
}
fn parse_litteral(input: &Vec<u8>, start: usize) -> Packet {
    let mut position = start + 6;
    let mut bin_value: Vec<u8> = Vec::new();
    while input[position] == 1 {
        bin_value.append(&mut input[position + 1..position + 5].to_vec());
        position += 5;
    }
    bin_value.append(&mut input[position + 1..position + 5].to_vec());
    // position += 5;
    let end = position + 5;
    let value = Some(to_u32(&bin_value));
    return Packet {
        version: to_u32(&input[start..start + 3].to_vec()) as u8,
        type_id: to_u32(&input[start + 3..start + 6].to_vec()) as u8,
        start: start,
        end: end,
        value: value,
        length_type_id: None,
        length: None,
        subpackets: vec![],
    };
}

fn to_u32(bin: &Vec<u8>) -> u64 {
    let mut val: u64 = 0;
    for (i, v) in bin.iter().rev().enumerate() {
        val += *v as u64 * u64::pow(2, i as u32);
    }
    val
}

fn read_input(filename: &str) -> Vec<u8> {
    let content = fs::read_to_string(filename).expect("can't read file");
    content
        .chars()
        .flat_map(|c| match c {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => vec![],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = read_input("input_test_1");
        assert_eq!(first(&input), 16);
        let input = read_input("input_test_2");
        assert_eq!(first(&input), 12);
        let input = read_input("input_test_3");
        assert_eq!(first(&input), 23);
        let input = read_input("input_test_4");
        assert_eq!(first(&input), 31);
    }
    #[test]
    fn test_2() {
        let input = read_input("input_test_5");
        assert_eq!(second(&input), 3);
        let input = read_input("input_test_6");
        assert_eq!(second(&input), 54);
        let input = read_input("input_test_7");
        assert_eq!(second(&input), 7);
        let input = read_input("input_test_8");
        assert_eq!(second(&input), 9);
        let input = read_input("input_test_9");
        assert_eq!(second(&input), 1);
        let input = read_input("input_test_10");
        assert_eq!(second(&input), 0);
        let input = read_input("input_test_11");
        assert_eq!(second(&input), 0);
        let input = read_input("input_test_12");
        assert_eq!(second(&input), 1);
    }
}
