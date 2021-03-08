// TODO

fn quick_sort<T: Ord + Copy>(left: usize, right: usize, arr: &mut [T]) {
    if right - left <= 0 {
        return;
    }

    let pivot: T = arr[left];
    let pivot_idx: usize = right / 2;
    quick_sort(left, (pivot_idx as isize - 1) as usize, arr);
    quick_sort(pivot_idx + 1, right, arr)
}

fn main() {
    let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];
    quick_sort(0, arr.len(), &mut arr);
    assert_eq!(arr, vec![-5, -2, 0, 0, 3, 8, 9, 12]);
}
