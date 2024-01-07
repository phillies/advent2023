#[derive(Debug, Clone)]
struct Field {
    fields: Vec<char>,
    width: usize,
    height: usize,
}

impl Field {
    fn find_row_symmetry(&self) -> Option<usize> {
        'row_loop: for row_index in 0..self.height - 1 {
            for column_index in 0..self.width {
                if self.fields[row_index * self.width + column_index]
                    != self.fields[(row_index + 1) * self.width + column_index]
                {
                    continue 'row_loop;
                }
            }
            return Some(row_index);
        }
        None
    }
}

pub fn solve(input: &Vec<String>) -> (i64, i64) {
    let mut fields = vec![];
    let mut field = Field {
        fields: vec![],
        width: input[0].len(),
        height: input[0].len(),
    };

    input.iter().for_each(|row| {
        if row.len() == 0 {
            fields.push(field.clone());
            field.fields.clear()
        } else {
            field.fields.extend(row.chars());
        }
    });
    fields.push(field);

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_dayxx() {
        let result_1 = 405;
        let result_2 = 0;
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
    fn bench_dayxx_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/dayxx.txt");
        b.iter(|| {
            solve(&input);
        });
    }
}
