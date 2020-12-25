use std::cmp::max;

/**
 * Bounded 0/1 Knapsack Problem
 * if j - weights[i] >= 0 -> dp[i][j] = max(dp[i-1][j], values[i] + dp[i-1][j - weights[i]])
 * if j - weights[i]  < 0 -> dp[i][j] = dp[i-1][j]
 */
pub fn bounded_01_knapsack(weights: &[i64], values: &[i64], capacity: i64) -> i64 {
    let mut dp: Vec<Vec<i64>> = vec![vec![0; capacity as usize + 1]; weights.len() + 1];

    for i in 1..weights.len() + 1 {
        for j in 1..capacity + 1 {
            if j - weights[i - 1] < 0 {
                dp[i][j as usize] = dp[i - 1][j as usize];
            } else {
                dp[i][j as usize] = max(dp[i - 1][j as usize], values[i - 1] + dp[i - 1][j as usize - weights[i - 1] as usize]);
            }
        }
    }

    // TODO: return items
    return dp[weights.len()][capacity as usize];
}

/**
 * Unbounded 0/1 Knapsack Problem
 * (The rod cutting problem is an instance of this problem)
 * dp[j] = 0 (initialization)
 * dp[j] = max(dp[j], values[i] + dp[j - weights[i]])
 */
pub fn unbounded_01_knapsack(weights: &[i64], values: &[i64], capacity: i64) -> i64 {
    let mut dp: Vec<i64> = vec![0; capacity as usize + 1];

    for i in 1..weights.len() + 1 {
        for j in 1..capacity + 1 {
            if j - weights[i - 1] >= 0 {
                dp[j as usize] = max(dp[j as usize], values[i - 1] + dp[j as usize - weights[i - 1] as usize]);
            }
        }
    }

    // TODO: return items
    return dp[capacity as usize];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_01_knapsack_test() {
        assert_eq!(bounded_01_knapsack(&([2, 1, 3, 2]), &([12, 10, 20, 15]), 5), 37);
        assert_eq!(bounded_01_knapsack(&([2, 5, 1, 3]), &([400, 500, 200, 350]), 6), 950);
    }

    #[test]
    fn unbounded_01_knapsack_test() {
        assert_eq!(unbounded_01_knapsack(&([1, 3, 4, 5]), &([10, 40, 50, 70]), 8), 110);
        assert_eq!(unbounded_01_knapsack(&([1, 2, 3, 4, 5, 6, 7, 8]), &([1, 5, 8, 9, 10, 17, 17, 20]), 4), 10);
    }
}