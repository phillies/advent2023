pub fn read_input_to_vector(filename: &str) -> Vec<String> {
    let input = std::fs::read_to_string(filename).expect("Could not read file!");
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.to_string());
    }
    result
}
