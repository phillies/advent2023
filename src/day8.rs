use num::integer::lcm;
use std::collections::BTreeMap;

fn build_map(input: Vec<String>) -> BTreeMap<String, (String, String)> {
    let mut map = BTreeMap::new();

    for line in input.iter() {
        let mut first_split = line.split(" = ");
        let source = first_split.next().unwrap().to_string();
        let mut second_split = first_split.next().unwrap().split(", ");
        let left = second_split.next().unwrap().replace("(", "");
        let right = second_split.next().unwrap().replace(")", "");
        map.insert(source, (left, right));
    }
    map
}

fn find_path(
    start_position: &String,
    instructions: &Vec<char>,
    map: &BTreeMap<String, (String, String)>,
    target_position: &String,
) -> u64 {
    let mut counter = 1;
    let mut instruction_pointer = 0;
    let mut current_position = start_position.clone();
    loop {
        let instruction: char = instructions[instruction_pointer];
        let (left, right) = map.get(&current_position).unwrap();
        match instruction {
            'R' => {
                current_position = right.clone();
            }
            'L' => {
                current_position = left.clone();
            }
            _ => {
                panic!("Unknown instruction {instruction}!")
            }
        }
        if current_position.ends_with(target_position) {
            break;
        }
        instruction_pointer += 1;
        instruction_pointer %= instructions.len();
        counter += 1;
    }
    counter
}

fn solve_1(input: &Vec<String>) -> u64 {
    let instructions = input[0].chars().collect::<Vec<char>>();

    let map = build_map(input[2..].to_vec());

    find_path(&"AAA".to_string(), &instructions, &map, &"ZZZ".to_string())
}

fn solve_2(input: &Vec<String>) -> u64 {
    let instructions = input[0].chars().collect::<Vec<char>>();

    let map = build_map(input[2..].to_vec());

    let starting_positions = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| (*k).clone())
        .collect::<Vec<String>>();

    let counter = starting_positions
        .iter()
        .map(|p| find_path(p, &instructions, &map, &"Z".to_string()))
        .collect::<Vec<u64>>();
    let steps = counter
        .iter()
        .fold(instructions.len() as u64, |acc, x| lcm(acc, *x));
    steps
}

pub fn solve(input: &Vec<String>) -> (u64, u64) {
    let result_1 = solve_1(&input);

    let result_2 = solve_2(&input);

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day8() {
        let result_1 = 2;
        let result_2 = 6;
        let input_1 = vec![
            "RL".to_string(),
            "".to_string(),
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];
        let input_2 = vec![
            "LR".to_string(),
            "".to_string(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ];
        let output_1 = solve_1(&input_1);
        let output_2 = solve_2(&input_2);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day8_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day8.txt");
        b.iter(|| {
            solve_1(&input);
        });
    }

    #[bench]
    fn bench_day8_part_2(b: &mut Bencher) {
        let input = read_input_to_vector("data/day8.txt");
        b.iter(|| {
            solve_2(&input);
        });
    }
}
