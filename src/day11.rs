/// That's my template for new days. Includes the solve function signature and a test function
#[derive(Debug)]
struct Galaxy {
    sectors: Vec<char>,
    width: usize,
    height: usize,
}

impl Galaxy {
    fn get(self: &Galaxy, x: usize, y: usize) -> char {
        *self.sectors.get(y * self.width + x).unwrap_or(&'.')
    }
    fn expand(self: &mut Galaxy) {
        let mut empty_rows = vec![];
        let mut empty_cols = vec![];
        for ii in 0..self.height {
            if self
                .sectors
                .get((ii * self.width..(ii + 1) * self.width))
                .unwrap()
                .iter()
                .all(|c| *c == '.')
            {
                empty_rows.push(ii);
            }
        }

        for ii in 0..self.width {
            if self
                .sectors
                .iter()
                .skip(ii)
                .step_by(self.width)
                .all(|c| *c == '.')
            {
                empty_cols.push(ii);
            }
        }

        println!("Empty rows: {:?}", empty_rows);
        println!("Empty cols: {:?}", empty_cols);
    }
}

pub fn solve(input: &Vec<String>) -> (i64, i64) {
    let mut galaxy = Galaxy {
        sectors: input.iter().flat_map(|s| s.chars()).collect::<Vec<char>>(),
        width: input[0].len(),
        height: input.len(),
    };
    galaxy.expand();
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day11() {
        let result_1 = 0;
        let result_2 = 0;
        let input = vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day11_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/dayx.txt");
        b.iter(|| {
            solve(&input);
        });
    }
}
