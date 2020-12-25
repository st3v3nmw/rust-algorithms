/**
 * Pick's Theorem (Area of Lattice Polygons)
 * Let P be a lattice polygon, let B(P) be the points on the boundary of the polygon,
 * and let I(P) be the number of points in the interior of the polygon.
 * Then the area of P = I(P) + 0.5 * B(P) - 1.
 */
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