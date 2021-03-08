/**
 * Longest Increasing Subsequence
 * (Strictly increasing subsequence)
 */
pub fn longest_increasing_subsequence<T: Ord + Copy>(v: &[T]) -> Vec<T> {
    let mut dp: Vec<(i64, i64)> = vec![(0, 0); v.len()];
    let mut max_elem_idx: usize = 0;
    for i in 0..dp.len() {
        let mut idx_max_len: i64 = -1;
        for j in 0..i {
            if v[i] > v[j] && (idx_max_len == -1 || dp[j].1 > dp[idx_max_len as usize].1) {
                idx_max_len = j as i64;
            }
        }

        if idx_max_len == -1 {
            dp[i].0 = i as i64;
            dp[i].1 = 1;
        } else {
            dp[i].0 = idx_max_len;
            dp[i].1 = dp[idx_max_len as usize].1 + 1;
        }

        // keep track of element with longest chain so far
        if dp[i].1 >= dp[max_elem_idx].1 {
            max_elem_idx = i;
        }
    }

    // backtrack from max_elem_idx
    let mut result: Vec<T> = vec![];
    loop {
        result.push(v[max_elem_idx]);

        if dp[max_elem_idx].1 == 1 {
            break;
        }

        max_elem_idx = dp[max_elem_idx].0 as usize;
    }
    result.reverse();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lis_test() {
        assert_eq!(
            longest_increasing_subsequence(&(vec![2, 0, 5, 3, 1, 4, 4, 5, 4, 6])),
            vec![2, 3, 4, 5, 6]
        );
        assert_eq!(
            longest_increasing_subsequence(&(vec!['Y', 'M', 'U', 'B', 'T', 'F', 'Y', 'S'])),
            vec!['B', 'F', 'S']
        );
    }
}
