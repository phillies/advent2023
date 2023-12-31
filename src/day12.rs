use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct SpringConfig {
    springs: Vec<char>,
    damaged_groups: Vec<u32>,
}

impl SpringConfig {
    fn check_match(&self, input: &Vec<char>) -> bool {
        true
    }

    fn find_variations(&self) -> i64 {
        let mut counter = 0;

        counter
    }
}

pub fn solve(input: &Vec<String>) -> (i64, i64) {
    let configs = input
        .iter()
        .map(|s| {
            let split = s.split(" ").collect::<Vec<&str>>();
            let record = split[0].chars().collect::<Vec<char>>();
            let config = split[1]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            SpringConfig {
                springs: record,
                damaged_groups: config,
            }
        })
        .collect::<Vec<SpringConfig>>();

    (21, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day12() {
        let result_1 = 21;
        let result_2 = 0;
        let input = vec![
            "???.### 1,1,3".to_string(),
            ".??..??...?##. 1,1,3".to_string(),
            "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
            "????.#...#... 4,1,1".to_string(),
            "????.######..#####. 1,6,5".to_string(),
            "?###???????? 3,2,1".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day12_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day12.txt");
        b.iter(|| {
            solve(&input);
        });
    }
}
