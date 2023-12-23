#![feature(array_chunks)]
#![feature(test)]
mod my_io;
use my_io::read_input_to_vector;
use std::{env, fs};

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn run(day: &String, input: &String) {
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
                "Sum of all IDs: {} - and the sum of power: {}",
                result_1, result_2
            );
        }
        "day3" => {
            let (result_1, result_2) = day3::solve(&read_input_to_vector(&input));
            println!(
                "Sum of all part numbers: {} - sum of all gear ratios: {}",
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
                "Lowest location number: {} - lowest number for ranges: {}",
                result_1, result_2
            );
        }
        "day6" => {
            let (result_1, result_2) = day6::solve(&read_input_to_vector(&input));
            println!(
                "Win product: {} - ways to win the long race: {}",
                result_1, result_2
            );
        }
        "day7" => {
            let (result_1, result_2) = day7::solve(&read_input_to_vector(&input));
            println!(
                "Total winnings w/ jacks: {} - and w/ jokers: {}",
                result_1, result_2
            );
        }
        "day8" => {
            let (result_1, result_2) = day8::solve(&read_input_to_vector(&input));
            println!(
                "Steps to reach ZZZ: {} - Steps to reach **Z: {}",
                result_1, result_2
            );
        }
        "day9" => {
            let (result_1, result_2) = day9::solve(&read_input_to_vector(&input));
            println!(
                "Sum of extrapolated next values {} - previous values {}",
                result_1, result_2
            );
        }
        "day10" => {
            let (result_1, result_2) = day10::solve(&read_input_to_vector(&input));
            println!("Maximum distance {} - inner tiles {}", result_1, result_2);
        }
        _ => {
            println!("No solution for this day yet!")
        }
    }
}
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        let directory_path = "data";
        let mut data_files = vec![];

        // Read the directory contents
        if let Ok(entries) = fs::read_dir(directory_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Get the file name
                    let file_name = entry.file_name();
                    data_files.push(file_name);
                }
            }
        } else {
            println!("Failed to read directory");
        }

        for file in data_files {
            let file_name = file.to_str().unwrap();
            let day = file_name.split(".").collect::<Vec<&str>>()[0];
            let input = format!("{}/{}", directory_path, file_name);
            run(&day.to_string(), &input);
        }
    } else {
        let day = args.get(1).expect("Please specify day as first argument!");
        let input = args
            .get(2)
            .expect("Please specify input as second argument!");

        run(&day, &input);
    }
}
