/// That's my template for new days. Includes the solve function signature and a test function
pub fn solve(input: Vec<String>) -> (u64, u64) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use crate::template::solve;

    #[test]
    fn test_solve() {
        let result_1 = 0;
        let result_2 = 0;
        let input = vec![
            "...".to_string(),
            "...".to_string(),
            "...".to_string(),
            "...".to_string(),
        ];
        let (output_1, output_2) = solve(input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }
}
