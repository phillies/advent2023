use substring::Substring;

/// to_digit() returns None when c is not a digit, so we can use find_map() to find the first digit
/// (from the beginning or reversed from the end)
fn find_digits(input_line: &str) -> u32 {
    let first_digit = input_line.chars().find_map(|c| c.to_digit(10));
    let last_digit = input_line.chars().rev().find_map(|c| c.to_digit(10));

    first_digit.expect("First digit failed to be recognized!") * 10
        + last_digit.expect("Last digit failed to be recognized!")
}

/// Plain and simple brute force solution of checking at each position starting
/// from the first or last character if its a digit or a number word. As soon as there
/// is a match, we break out of the loop.
fn find_digits_or_words(input_line: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    for ii in 0..input_line.len() {
        let c = input_line.chars().nth(ii).unwrap();
        if c.is_digit(10) {
            first_digit = c.to_digit(10);
            break;
        } else if input_line.substring(ii, ii + 3) == "one" {
            first_digit = Some(1);
            break;
        } else if input_line.substring(ii, ii + 3) == "two" {
            first_digit = Some(2);
            break;
        } else if input_line.substring(ii, ii + 5) == "three" {
            first_digit = Some(3);
            break;
        } else if input_line.substring(ii, ii + 4) == "four" {
            first_digit = Some(4);
            break;
        } else if input_line.substring(ii, ii + 4) == "five" {
            first_digit = Some(5);
            break;
        } else if input_line.substring(ii, ii + 3) == "six" {
            first_digit = Some(6);
            break;
        } else if input_line.substring(ii, ii + 5) == "seven" {
            first_digit = Some(7);
            break;
        } else if input_line.substring(ii, ii + 5) == "eight" {
            first_digit = Some(8);
            break;
        } else if input_line.substring(ii, ii + 4) == "nine" {
            first_digit = Some(9);
            break;
        }
    }
    let mut last_digit = None;
    for ii in (0..input_line.len()).rev() {
        let c = input_line.chars().nth(ii).unwrap();
        if c.is_digit(10) {
            last_digit = c.to_digit(10);
            break;
        } else if input_line.substring(ii, ii + 3) == "one" {
            last_digit = Some(1);
            break;
        } else if input_line.substring(ii, ii + 3) == "two" {
            last_digit = Some(2);
            break;
        } else if input_line.substring(ii, ii + 5) == "three" {
            last_digit = Some(3);
            break;
        } else if input_line.substring(ii, ii + 4) == "four" {
            last_digit = Some(4);
            break;
        } else if input_line.substring(ii, ii + 4) == "five" {
            last_digit = Some(5);
            break;
        } else if input_line.substring(ii, ii + 3) == "six" {
            last_digit = Some(6);
            break;
        } else if input_line.substring(ii, ii + 5) == "seven" {
            last_digit = Some(7);
            break;
        } else if input_line.substring(ii, ii + 5) == "eight" {
            last_digit = Some(8);
            break;
        } else if input_line.substring(ii, ii + 4) == "nine" {
            last_digit = Some(9);
            break;
        }
    }

    first_digit.expect("First digit failed to be recognized!") * 10
        + last_digit.expect("Last digit failed to be recognized!")
}

pub fn part_one(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        sum += find_digits(&line);
    }
    return sum;
}

pub fn part_two(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        sum += find_digits_or_words(&line);
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::day1::{part_one, part_two};

    #[test]
    fn step_one() {
        let result = 142;
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        let output = part_one(input);
        assert_eq!(result, output);
    }

    #[test]
    fn step_two() {
        let result = 281;
        let input = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        let output = part_two(input);
        assert_eq!(result, output);
    }
}
