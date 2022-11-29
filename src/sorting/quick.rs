fn quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        quick_sort(arr, low, p-1);
        quick_sort(arr, p + 1, high);
    }
}

fn partition<T:Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;
    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];
        let len = arr.len();
        quick_sort(&mut arr, 0, (len - 1) as isize);
        assert_eq!(arr, vec![-5, -2, 0, 0, 3, 8, 9, 12]);
    }
}
