/**
    Generating subsets (O(2^n) time).
**/
pub fn subsets<T: Copy>(set: &[T]) -> Vec<Vec<T>> {
    let mut sets: Vec<Vec<T>> = vec![];
    for mut n in 0..2_i32.pow(set.len() as u32) {
        let mut pos = 0;
        let mut curr: Vec<T> = vec![];
        while n > 0 {
            if n % 2 == 1 {
                curr.push(set[pos]);
            }
            pos += 1;
            n /= 2;
        }
        sets.push(curr);
    }

    return sets;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets_test() {
        assert_eq!(subsets(&(['A', 'B', 'C'])), [vec![], vec!['A'], vec!['B'], vec!['A', 'B'], vec!['C'], vec!['A', 'C'], vec!['B', 'C'], vec!['A', 'B', 'C']]);
    }
}