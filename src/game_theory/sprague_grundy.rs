fn sprague_grundy(n: i32, nimbres: &mut [i32], moves: &[i32]) -> i32 {
    if nimbres[n as usize] != -1 {
        return nimbres[n as usize];
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

fn main() {
    let mut v = [-1; 7];
    v[0] = 0;
    sprague_grundy(6, &mut v, &(vec![1, 4]));
    assert_eq!(v, [0, 1, 0, 1, 2, 0, 1]);
}