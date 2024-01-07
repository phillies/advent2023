#[derive(Debug, Clone)]
struct SpringConfig {
    springs: Vec<char>,
    damaged_groups: Vec<u32>,
}

impl SpringConfig {
    fn create_variations(&self, springs: &Vec<char>, index: usize) -> Vec<Vec<char>> {
        if index >= springs.len() {
            if self.check_match(springs) {
                return vec![springs.clone().to_vec()];
            } else {
                return vec![];
            }
        }

        match springs[index] {
            '?' => {
                let mut variant_1 = springs.clone();
                variant_1[index] = '.';
                let mut variant_2 = springs.clone();
                variant_2[index] = '#';
                let mut result = vec![];
                if self.check_submatch(&variant_1.get(0..index).unwrap().to_vec()) {
                    let mut result_1 = self.create_variations(&variant_1, index + 1);
                    result.append(&mut result_1)
                }
                if self.check_submatch(&variant_2.get(0..index).unwrap().to_vec()) {
                    let mut result_2 = self.create_variations(&variant_2, index + 1);
                    result.append(&mut result_2);
                }
                result
            }
            _ => self.create_variations(springs, index + 1),
        }
    }
    fn check_submatch(&self, input: &Vec<char>) -> bool {
        let string_representation = input.iter().collect::<String>();
        let blocks = string_representation
            .split('.')
            .filter(|x| x.len() > 0)
            .collect::<Vec<&str>>();
        if blocks.len() < 1 {
            return true;
        }
        if blocks.len() > self.damaged_groups.len() {
            return false;
        }
        for (index, b) in blocks
            .get(0..blocks.len() - 1)
            .unwrap()
            .into_iter()
            .enumerate()
        {
            if b.len() != self.damaged_groups[index] as usize {
                return false;
            }
        }
        true
    }
    fn check_match(&self, input: &Vec<char>) -> bool {
        let string_representation = input.iter().collect::<String>();
        let blocks = string_representation
            .split('.')
            .filter(|x| x.len() > 0)
            .collect::<Vec<&str>>();
        if blocks.len() != self.damaged_groups.len() {
            return false;
        }

        for (index, length) in self.damaged_groups.iter().enumerate() {
            if blocks[index].len() != (*length as usize) {
                return false;
            }
        }
        true
    }

    fn find_variations(&self) -> i64 {
        self.create_variations(&self.springs, 0).into_iter().count() as i64
    }

    fn find_unfold_variations(&self, number_of_folds: u32) -> i64 {
        if number_of_folds == 0 {
            0
        } else {
            let base_variations = self.find_variations();
            let mut temp = self.clone();
            temp.damaged_groups.extend(self.damaged_groups.iter());
            temp.springs.push('?');
            temp.springs.extend(self.springs.iter());
            let fold_variations =
                temp.create_variations(&temp.springs, 0).into_iter().count() as i64;
            base_variations * (fold_variations / base_variations).pow(number_of_folds - 1)
        }
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

    let res = configs
        .iter()
        .map(|c| c.find_variations())
        .collect::<Vec<i64>>()
        .iter()
        .sum();

    let res_fold = configs
        .iter()
        .map(|c| c.find_unfold_variations(5))
        .collect::<Vec<i64>>()
        .iter()
        .sum();

    (res, res_fold)
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
        let result_2 = 525152;
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
