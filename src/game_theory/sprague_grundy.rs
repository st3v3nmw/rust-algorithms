/**
 * Sprague Grundy Theorem for combinatorial games like Nim
 * Let G(u) be the Grundy number of a pile with u stones. The  Grundy number of a terminal state is 0;
 * otherwise, G(u) is recursively defined as x = mex{x_1,...,x_k} where x_i is the Grundy value for state v_i and
 * the function mex (minimum excludant) is the smallest non-negative integer not found in the given set.
*/
pub fn sprague_grundy(n: i64, nimbres: &mut [i64], moves: &[i64]) -> i64 {
    if nimbres[n as usize] != -1 {
        return nimbres[n as usize];
    }
    if n == 0 { // terminal state
        nimbres[0] = 0;
        return 0;
    }

    let mut set: Vec<i64> = vec![];
    for mv in moves.iter() {
        if n - mv >= 0 {
            set.push(sprague_grundy(n - mv, nimbres, moves));
        }
    }

    set.sort_unstable();
    // set.dedup();
    let mut mex: i64 = 0;
    for ht in set.iter() {
        if *ht != mex {
            break;
        }
        mex += 1;
    }
    nimbres[n as usize] = mex;
    return mex;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprague_grundy_test() {
        let mut v: Vec<i64> = vec![-1; 7];
        sprague_grundy(6, &mut v, &(vec![1, 4]));
        assert_eq!(v, [0, 1, 0, 1, 2, 0, 1]);
    }
}