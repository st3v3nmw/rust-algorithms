pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_test() {
        let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![-5, -2, 0, 0, 3, 8, 9, 12]);
    }
}