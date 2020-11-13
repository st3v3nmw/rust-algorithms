/**
    Coin Change Problem
    Given denominations, provide the required change using the least number of coins.
    dp[0] = 0
    dp[i] = min(dp[i - denominations[j]]) + 1
**/
fn coin_change(denominations: &mut Vec<i64>, change: i64) -> Vec<i64> {
    let mut dp: Vec<(i64, i64)> = vec![(i64::MAX, 0); change as usize + 1];
    dp[0].0 = 0;

    for i in 1..change + 1 {
        for j in 0..denominations.len() {
            let idx: i64 = i - denominations[j];
            if idx >= 0 && dp[i as usize].0 > dp[idx as usize].0 {
                dp[i as usize].0 = dp[idx as usize].0;
                dp[i as usize].1 = denominations[j];
            }
        }
        dp[i as usize].0 += 1;
    }

    // get denominations
    let mut picked: Vec<i64> = vec![];
    let mut idx: i64 = dp.len() as i64 - 1;
    while idx > 0 {
        picked.push(dp[idx as usize].1);
        idx -= dp[idx as usize].1;
    }
    return picked;
}

fn main() {
    assert_eq!(coin_change(&mut (vec![1, 3, 4]), 6), vec![3, 3]);
}