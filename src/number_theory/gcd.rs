// Compute the GCD of m & n using Euclid's Algorithm
pub fn gcd(m: i64, n: i64) -> i64 {
    if n == 0 {
        return m.abs();
    }
    return gcd(n, m % n);
}

// Compute the GCD of a list
pub fn gcd_arr(v: &[i64]) -> i64 {
    let mut gcd_r = v[0];
    for num in v {
        gcd_r = gcd(gcd_r, *num);
    }
    return gcd_r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(225, 144), 9);
        assert_eq!(gcd(144, -225), 9);
        assert_eq!(gcd(-144, 225), 9);
        assert_eq!(gcd(-144, -225), 9);
    }

    #[test]
    fn gcd_arr_test() {
        assert_eq!(gcd_arr(&mut (vec![200, 500, 6000])), 100);
    }
}
