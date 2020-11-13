fn counting_sort(arr: &[i32], lower_bound: i32, upper_bound: i32) -> Vec<i32> {
    let mut count_arr: Vec<i32> = vec![0; (upper_bound - lower_bound) as usize];
    for e in arr {
        count_arr[(*e - lower_bound) as usize] += 1;
    }

    let mut sorted: Vec<i32> = vec![];
    for i in 0..count_arr.len() {
        for _ in 0..count_arr[i] {
            sorted.push(lower_bound + i as i32);
        }
    }
    return sorted;
}

fn main() {
    let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];
    assert_eq!(counting_sort(&mut arr, -5, 15), vec![-5, -2, 0, 0, 3, 8, 9, 12]);
}