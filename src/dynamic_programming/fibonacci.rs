#[macro_export] macro_rules! ternary { ($condition: expr, $_true: expr, $_false: expr) => { if $condition { $_true } else { $_false } }; }

/**
 * Fibonacci Sequence
 * f[0] = 0
 * f[1] = 1
 * f[n] = f[n-1] + f[n-2]
 */
pub fn fibonacci_iterative(n: i64) -> i64 { // O(n) time
    let mut f: Vec<i64> = vec![0; n as usize + 1];
    f[1] = 1;

    for i in 2..n + 1 {
        f[i as usize] = f[i as usize - 1] + f[i as usize - 2];
    }

    return f[n as usize];
}

pub fn fibonacci_recursive_wrapper(n: i64) -> i64 {
    let mut dp: Vec<i64> = vec![-1; n as usize + 1];
    dp[0] = 0;
    dp[1] = 1;
    return fibonacci_recursive(n, &mut dp);
}

pub fn fibonacci_recursive(n: i64, dp: &mut [i64]) -> i64 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let f1: i64 = ternary!(dp[n as usize - 1] == -1, fibonacci_recursive(n - 1, dp), dp[n as usize - 1]);
    let f2: i64 = ternary!(dp[n as usize - 2] == -1, fibonacci_recursive(n - 2, dp), dp[n as usize - 2]);

    dp[n as usize] = f1 + f2; // memoize
    return dp[n as usize];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_iterative_test() {
        assert_eq!(fibonacci_iterative(40), 102334155);
    }

    #[test]
    fn fibonacci_recursive_test() {
        assert_eq!(fibonacci_recursive_wrapper(40), 102334155);
    }
}