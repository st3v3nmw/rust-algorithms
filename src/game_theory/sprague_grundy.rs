pub fn sprague_grundy(n: i32, nimbres: &mut [i32], moves: &[i32]) -> i32 {
    if nimbres[n as usize] != -1 {
        return nimbres[n as usize];
    }
    if n == 0 { // terminal state
        nimbres[0] = 0;
        return 0;
    }

    let mut set: Vec<i32> = vec![];
    for mv in moves.iter() {
        if n - mv >= 0 {
            set.push(sprague_grundy(n - mv, nimbres, moves));
        }
    }

    set.sort_unstable();
    // set.dedup();
    let mut mex: i32 = 0;
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
        let mut v: Vec<i32> = vec![-1; 7];
        sprague_grundy(6, &mut v, &(vec![1, 4]));
        assert_eq!(v, [0, 1, 0, 1, 2, 0, 1]);
    }
}