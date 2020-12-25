use std::cmp::max;

/**
 * Kadane's algorithm (O(n) time)
 * Maximum Subarray Problem.
 * dp[0] = arr[0]
 * dp[i] = max(dp[i-1], 0) + arr[i]
 */
pub fn kadane(arr: &[i64]) -> i64 {
    let mut dp: Vec<i64> = vec![0; arr.len()];
    dp[0] = arr[0];
    let mut mmax: i64 = dp[0];
    for i in 1..arr.len() {
        dp[i] = max(dp[i - 1], 0) + arr[i];
        mmax = max(mmax, dp[i]);
    }
    return mmax;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kadane_test() {
        assert_eq!(kadane(&([-3, 1, -8, 12, 0, -3, 5, -9, 4])), 14);
    }
}