pub fn picks_theorem(internal_points: i64, border_points: i64) -> i64 {
    return internal_points + (border_points as f64 / 2.0) as i64 - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_theorem_test() {
        assert_eq!(picks_theorem(0, 4), 1);
        assert_eq!(picks_theorem(22, 24), 33);
    }
}