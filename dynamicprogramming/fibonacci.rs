/**
    f[0] = 0
    f[1] = 1
    f[n] = f[n-1] + f[n-2]
**/

fn fibonacci_iterative(n: i64) -> i64 {
    let mut f: Vec<i64> = vec![0; n as usize + 1];
    f[1] = 1;

    for i in 2..n + 1 {
        f[i as usize] = f[i as usize - 1] + f[i as usize - 2];
    }

    return f[n as usize];
}

fn fibonacci_recursive_wrapper(n: i64) -> i64 {
    let mut dp: Vec<i64> = vec![-1; n as usize + 1];
    dp[0] = 0;
    dp[1] = 1;
    return fibonacci_recursive(n, &mut dp);
}

fn fibonacci_recursive(n: i64, dp: &mut [i64]) -> i64 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let (f1, f2): (i64, i64);
    if dp[n as usize - 1] == -1 {
        f1 = fibonacci_recursive(n-1, dp);
    } else {
        f1 = dp[n as usize - 1];
    }

    if dp[n as usize - 2] == -1 {
        f2 = fibonacci_recursive(n-2, dp);
    } else {
        f2 = dp[n as usize - 2];
    }

    dp[n as usize] = f1 + f2;
    return dp[n as usize];
}

fn main() {
    // Iterative Fibonacci
    assert_eq!(fibonacci_iterative(40), 102334155);

    // Recursive Fibonacci
    assert_eq!(fibonacci_recursive_wrapper(40), 102334155);
}