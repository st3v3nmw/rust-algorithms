pub fn counting_sort(arr: &[i64], lower_bound: i64, upper_bound: i64) -> Vec<i64> {
    let mut count_arr: Vec<i64> = vec![0; (upper_bound - lower_bound) as usize];
    for e in arr {
        count_arr[(*e - lower_bound) as usize] += 1;
    }

    let mut sorted: Vec<i64> = vec![];
    for i in 0..count_arr.len() {
        for _ in 0..count_arr[i] {
            sorted.push(lower_bound + i as i64);
        }
    }
    return sorted;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_sort_test() {
        let mut arr: Vec<i64> = vec![3, -2, 9, 0, 12, -5, 8, 0];
        assert_eq!(
            counting_sort(&mut arr, -5, 15),
            vec![-5, -2, 0, 0, 3, 8, 9, 12]
        );
    }
}
