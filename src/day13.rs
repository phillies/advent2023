#[derive(Debug, Clone)]
struct Field {
    fields: Vec<char>,
    width: usize,
    height: usize,
}

impl Field {
    fn get(&self, x: usize, y: usize) -> char {
        self.fields[y * self.width + x]
    }

    fn check_row_symmetry(&self, start_row: usize) -> bool {
        let upper_half = (0..(start_row + 1)).rev();
        let lower_half = (start_row + 1)..self.height;
        for (upper, lower) in upper_half.into_iter().zip(lower_half.into_iter()) {
            for column_index in 0..self.width {
                if self.get(column_index, upper) != self.get(column_index, lower) {
                    return false;
                }
            }
        }
        true
    }

    fn check_row_symmetry_with_smudge(&self, start_row: usize) -> bool {
        let upper_half = (0..(start_row + 1)).rev();
        let lower_half = (start_row + 1)..self.height;
        let mut number_of_smudges = 0;

        for (upper, lower) in upper_half.into_iter().zip(lower_half.into_iter()) {
            let mut smudge = false;

            for column_index in 0..self.width {
                if self.get(column_index, upper) != self.get(column_index, lower) {
                    if smudge {
                        return false;
                    } else {
                        smudge = true;
                        number_of_smudges += 1;
                    }
                }
            }
        }

        number_of_smudges == 1
    }

    fn find_row_symmetry(&self, tolerate_smudge: bool) -> Option<usize> {
        let symmetry_check = if tolerate_smudge {
            Field::check_row_symmetry_with_smudge
        } else {
            Field::check_row_symmetry
        };
        'row_loop: for row_index in 0..self.height - 1 {
            let mut smudge = false;
            for column_index in 0..self.width {
                if self.get(column_index, row_index) != self.get(column_index, row_index + 1) {
                    if tolerate_smudge {
                        if smudge {
                            continue 'row_loop;
                        } else {
                            smudge = true;
                        }
                    } else {
                        continue 'row_loop;
                    }
                }
            }
            if symmetry_check(self, row_index) {
                return Some(row_index);
            }
        }
        None
    }

    fn check_column_symmetry(&self, start_column: usize) -> bool {
        let left_half = (0..(start_column + 1)).rev();
        let right_half = (start_column + 1)..self.width;
        for (left, right) in left_half.into_iter().zip(right_half.into_iter()) {
            for row_index in 0..self.height {
                if self.get(left, row_index) != self.get(right, row_index) {
                    return false;
                }
            }
        }
        true
    }

    fn check_column_symmetry_with_smudge(&self, start_column: usize) -> bool {
        let left_half = (0..(start_column + 1)).rev();
        let right_half = (start_column + 1)..self.width;
        let mut number_of_smudges = 0;
        for (left, right) in left_half.into_iter().zip(right_half.into_iter()) {
            let mut smudge = false;
            for row_index in 0..self.height {
                if self.get(left, row_index) != self.get(right, row_index) {
                    if smudge {
                        return false;
                    } else {
                        smudge = true;
                        number_of_smudges += 1;
                    }
                }
            }
        }

        number_of_smudges == 1
    }

    fn find_column_symmetry(&self, tolerate_smudge: bool) -> Option<usize> {
        let symmetry_check = if tolerate_smudge {
            Field::check_column_symmetry_with_smudge
        } else {
            Field::check_column_symmetry
        };
        'column_loop: for column_index in 0..self.width - 1 {
            let mut smudge = false;
            for row_index in 0..self.height {
                if self.get(column_index, row_index) != self.get(column_index + 1, row_index) {
                    if tolerate_smudge {
                        if smudge {
                            continue 'column_loop;
                        } else {
                            smudge = true;
                        }
                    } else {
                        continue 'column_loop;
                    }
                }
            }
            if symmetry_check(self, column_index) {
                return Some(column_index);
            }
        }
        None
    }
}

fn fields_from_input(input: &Vec<String>) -> Vec<Field> {
    let mut fields = vec![];
    let mut field = Field {
        fields: vec![],
        width: input[0].len(),
        height: 0,
    };

    input.iter().for_each(|row| {
        if row.len() == 0 {
            fields.push(field.clone());
            field.fields.clear();
            field.height = 0;
        } else {
            field.width = row.len();
            field.fields.extend(row.chars());
            field.height += 1;
        }
    });
    fields.push(field);
    fields
}

pub fn solve(input: &Vec<String>) -> (i64, i64) {
    let fields = fields_from_input(input);
    let mut reflection_sum = 0;
    fields.iter().for_each(|field| {
        if let Some(row_index) = field.find_row_symmetry(false) {
            reflection_sum += 100 * (row_index + 1) as i64;
        } else if let Some(column_index) = field.find_column_symmetry(false) {
            reflection_sum += (column_index + 1) as i64;
        }
    });

    let mut reflection_sum_with_smudge = 0;
    fields.iter().for_each(|field| {
        if let Some(row_index) = field.find_row_symmetry(true) {
            reflection_sum_with_smudge += 100 * (row_index + 1) as i64;
        } else if let Some(column_index) = field.find_column_symmetry(true) {
            reflection_sum_with_smudge += (column_index + 1) as i64;
        } else {
            panic!("No symmetry found");
        }
    });

    (reflection_sum, reflection_sum_with_smudge)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day13() {
        let result_1 = 405;
        let result_2 = 400;
        let input = vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
            "".to_string(),
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day13_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day13.txt");
        b.iter(|| {
            solve(&input);
        });
    }
}
