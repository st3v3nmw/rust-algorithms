use std::cmp::max;

/**
    Coin row problem
    Pick a subsequence of non-adjacent coins with the maximum total value.
    dp[0] = 0
    dp[1] = coins[1]
    dp[i] = max(coins[i] + dp[i-2], dp[i-1])
**/
fn coin_row(coins: &mut Vec<i64>) -> (Vec<i64>, i64) {
    let mut dp: Vec<i64> = vec![0; coins.len() + 1];
    coins.insert(0, 0);
    dp[1] = coins[1];
    for i in 2..coins.len() {
        dp[i] = max(coins[i] + dp[i - 2], dp[i - 1]);
    }
    let total: i64 = dp[dp.len() - 1];

    // get coins
    let mut i: i32= dp.len() as i32 - 1;
    let mut picked: Vec<i64> = vec![];
    while i >= 0 {
        if dp[i as usize] == dp[(i - 1) as usize] {
            i -= 1;
        } else {
            picked.push(coins[i as usize]);
            i -= 2;
        }
    }
    picked.reverse();

    return (picked, total);
}

fn main() {
    assert_eq!(coin_row(&mut (vec![5, 1, 2, 10, 6, 2])), (vec![5, 10, 2], 17));
}