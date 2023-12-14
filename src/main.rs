#![feature(array_chunks)]
#![feature(test)]
mod my_io;
use my_io::read_input_to_vector;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Please specify day as first argument!");
    let input = env::args()
        .nth(2)
        .expect("Please specify input as second argument!");

    match day.as_str() {
        "day1" => {
            let input = &read_input_to_vector(&input);
            let result = day1::part_one(input);
            let result_2 = day1::part_two(input);
            println!(
                "Sum of all calibrations: part 1: {} - part 2: {}",
                result, result_2
            );
        }
        "day2" => {
            let (result_1, result_2) = day2::solve(&read_input_to_vector(&input));
            println!(
                "Sum of all ids: {} - and their power: {}",
                result_1, result_2
            );
        }
        "day3" => {
            let (result_1, result_2) = day3::solve(&read_input_to_vector(&input));
            println!(
                "Sum of all part numbers: {} - gear ratios: {}",
                result_1, result_2
            );
        }
        "day4" => {
            let (result_1, result_2) = day4::solve(&read_input_to_vector(&input));
            println!("total points: {} - total cards: {}", result_1, result_2);
        }
        "day5" => {
            let (result_1, result_2) = day5::solve(&read_input_to_vector(&input));
            println!(
                "Smallest location: {} - and their power: {}",
                result_1, result_2
            );
        }
        "day6" => {
            let (result_1, result_2) = day6::solve(&read_input_to_vector(&input));
            println!("Win product: {} - and their power: {}", result_1, result_2);
        }
        "day7" => {
            let (result_1, result_2) = day7::solve(&read_input_to_vector(&input));
            println!("Step 1: {} - and their power: {}", result_1, result_2);
        }
        "day8" => {
            let (result_1, result_2) = day8::solve(&read_input_to_vector(&input));
            println!("Step 1: {} - and their power: {}", result_1, result_2);
        }
        _ => {
            println!("day not found");
        }
    }
}
