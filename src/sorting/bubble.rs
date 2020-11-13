pub fn bubble_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len: usize = arr.len();
    for _ in 0..len {
        for i in 0..len-1 {
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![-5, -2, 0, 0, 3, 8, 9, 12]);
    }
}