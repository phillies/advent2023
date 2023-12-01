use std::env;

mod day1;

fn read_input_to_vector(filename: &str) -> Vec<String> {
    let input = std::fs::read_to_string(filename).expect("Could not read file!");
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.to_string());
    }
    result
}

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Please specify day as first argument!");
    let input = env::args()
        .nth(2)
        .expect("Please specify input as second argument!");

    match day.as_str() {
        "day1" => {
            let result = day1::part_one(read_input_to_vector(&input));
            let result_2 = day1::part_two(read_input_to_vector(&input));
            println!(
                "Sum of all calibrations: part 1: {} - part 2: {}",
                result, result_2
            );
        }
        _ => {
            println!("day not found");
        }
    }
}
