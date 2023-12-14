use std::collections::BTreeMap;

#[derive(Debug)]
struct Engine {
    parts: BTreeMap<Position, char>,
    numbers: BTreeMap<Position, u64>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
struct Position {
    // We take signed integer so we can represent negative positions (above the first row and before the first column)
    y: i64,
    x: i64,
}

fn is_neighbor(position: &Position, value: u64, other: &Position) -> bool {
    let y_diff = (position.y - other.y).abs();
    (y_diff <= 1)
        && (other.x >= position.x - 1)
        && (other.x <= (position.x + value.to_string().len() as i64))
}

fn parse_engine(input_lines: &Vec<String>) -> Engine {
    let mut engine = Engine {
        parts: BTreeMap::new(),
        numbers: BTreeMap::new(),
    };

    for (p_y, engine_line) in input_lines.into_iter().enumerate() {
        let mut parsing_number_in_progress = false;
        let mut parsing_number_start_positon = Position { x: 0, y: 0 };
        let mut parsed_value = 0u64;
        for (p_x, entry) in engine_line.chars().enumerate() {
            // We cannot be sure if usize fits into i64 at compile time, but we don't expect such long liines
            // and we can take the struct initialization shortcut with variable names x and y
            let x: i64 = p_x.try_into().unwrap();
            let y: i64 = p_y.try_into().unwrap();
            match entry {
                entry if entry.is_ascii_digit() => {
                    if parsing_number_in_progress {
                        parsed_value *= 10;
                    } else {
                        parsing_number_in_progress = true;
                        parsing_number_start_positon = Position { x, y };
                        parsed_value = 0;
                    }
                    parsed_value += entry.to_digit(10).unwrap() as u64;
                }
                '.' => {
                    if parsing_number_in_progress {
                        parsing_number_in_progress = false;
                        engine
                            .numbers
                            .insert(parsing_number_start_positon, parsed_value);
                    }
                }
                entry if entry.is_ascii() => {
                    if parsing_number_in_progress {
                        parsing_number_in_progress = false;
                        engine
                            .numbers
                            .insert(parsing_number_start_positon, parsed_value);
                    }
                    engine.parts.insert(Position { x, y }, entry);
                }
                _ => {
                    // Should only happen if there is a problem with the input data
                }
            }
        }
        if parsing_number_in_progress {
            engine
                .numbers
                .insert(parsing_number_start_positon, parsed_value);
        }
    }
    engine
}

fn calculate_part_sum(engine: &Engine) -> u64 {
    let mut part_sum = 0;
    'numbers: for (pos, value) in engine.numbers.iter() {
        let value_length: i64 = value.to_string().chars().count().try_into().unwrap();
        for x in (pos.x - 1)..(pos.x + value_length + 1) {
            for y in (pos.y - 1)..(pos.y + 2) {
                if engine.parts.contains_key(&Position { x, y }) {
                    part_sum += value;
                    continue 'numbers;
                }
            }
        }
    }
    part_sum
}

fn calculate_gear_ratios(engine: &Engine) -> u64 {
    let mut gear_ratios = 0;
    for (pos, _) in engine.parts.iter().filter(|(_, v)| **v == '*') {
        let gears = engine
            .numbers
            .iter()
            .filter(|(p, v)| is_neighbor(p, **v, pos))
            .map(|(_, v)| *v)
            .collect::<Vec<u64>>();
        if gears.len() == 2 {
            gear_ratios += gears[0] * gears[1];
        }
    }
    gear_ratios
}

pub fn solve(input_lines: &Vec<String>) -> (u64, u64) {
    let engine = parse_engine(input_lines);

    let part_sum = calculate_part_sum(&engine);

    let gear_ratios = calculate_gear_ratios(&engine);
    (part_sum, gear_ratios)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day3() {
        let result_1 = 4361;
        let result_2 = 467835;
        let input = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day3_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day3.txt");
        b.iter(|| {
            let engine = parse_engine(&input);
            calculate_part_sum(&engine);
        });
    }

    #[bench]
    fn bench_day3_part_2(b: &mut Bencher) {
        let input = read_input_to_vector("data/day3.txt");
        b.iter(|| {
            let engine = parse_engine(&input);
            calculate_gear_ratios(&engine);
        });
    }
}
