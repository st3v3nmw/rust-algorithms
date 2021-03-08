pub fn selection_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len: usize = arr.len();
    for i in 0..len {
        let idx: usize = (i..len).min_by_key(|x| arr[*x]).unwrap();
        arr.swap(i, idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_sort_test() {
        let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![-5, -2, 0, 0, 3, 8, 9, 12]);
    }
}
