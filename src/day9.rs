use std::iter::zip;

fn predict_next(input: &Vec<i64>) -> i64 {
    if input.len() < 2 {
        return input[0];
    }
    if input.iter().all(|x| *x == input[0]) {
        return input[0];
    } else {
        let next_layer = zip(input.iter(), input.iter().skip(1))
            .map(|(x, y)| y - x)
            .collect::<Vec<i64>>();
        let prediction = predict_next(&next_layer);
        return input.last().unwrap() + prediction;
    }
}

fn predict_previous(input: &Vec<i64>) -> i64 {
    if input.len() < 2 {
        return input[0];
    }
    if input.iter().all(|x| *x == input[0]) {
        return input[0];
    } else {
        let next_layer = zip(input.iter(), input.iter().skip(1))
            .map(|(x, y)| y - x)
            .collect::<Vec<i64>>();
        let prediction = predict_previous(&next_layer);
        return input.first().unwrap() - prediction;
    }
}

pub fn solve(input: &Vec<String>) -> (i64, i64) {
    let mut sum_next = 0;
    let mut sum_previous = 0;
    for line in input.iter() {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        sum_next += predict_next(&numbers);
        sum_previous += predict_previous(&numbers);
    }
    (sum_next, sum_previous)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day9_both() {
        let result_1 = 114;
        let result_2 = 2;
        let input = vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day9_both(b: &mut Bencher) {
        let input = read_input_to_vector("data/day9.txt");
        b.iter(|| {
            solve(&input);
        });
    }
}
