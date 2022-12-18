// TODO: Bucket Sort
struct Bucket<H, T> {
    hash: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    pub fn new(hash: H, value: T) -> Bucket<H, T> {
        Bucket { hash, values: vec![value], }
    }    
}

pub fn bucket_sort<H, F, T>(arr: &mut [T], hasher: F) where 
    H: Ord, 
    F: Fn(&T) -> H,
    T: Ord + Clone 
{
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();
    for value in arr.iter() {
        let hash = hasher(&value);
        let value = value.clone();
        match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
            Ok(index) => buckets[index].values.push(value),
            Err(index) => buckets.insert(index, Bucket::new(hash, value))            
        }
    }
    let ret = buckets.into_iter().flat_map(|mut bucket| { bucket.values.sort(); bucket.values}).collect::<Vec<T>>();
    arr.clone_from_slice(&ret);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bucket_sort_test() {
        let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];
        // TODO: hasher function
        bucket_sort(&mut arr, hasher);
        assert_eq!(arr, vec![-5, -2, 0, 0, 3, 8, 9, 12]);
    }
}
