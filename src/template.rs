/// That's my template for new days. Includes the solve function signature and a test function
pub fn solve(input: &Vec<String>) -> (i64, i64) {
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
        let result_1 = 0;
        let result_2 = 0;
        let input = vec![
            "...".to_string(),
            "...".to_string(),
            "...".to_string(),
            "...".to_string(),
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
