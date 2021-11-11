pub fn ints(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
