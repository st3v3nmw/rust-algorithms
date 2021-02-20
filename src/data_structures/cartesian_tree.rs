#[derive(Debug)]
struct CartesianTreeNode {
    value: i64,
    left: Option<Box<CartesianTreeNode>>,
    right: Option<Box<CartesianTreeNode>>
}

impl CartesianTreeNode {
    fn new(v: &[i64]) -> Self {
        let mut max_idx: usize = 0;
        let mut max_val: i64 = 0;
        for i in 0..v.len() {
            if v[i as usize] > max_val {
                max_val = v[i];
                max_idx = i as usize;
            }
        }
        let mut left = None;
        if v[0..max_idx].len() > 0 {
            left = Some(Box::new(Self::new(&v[0..max_idx])));
        }

        let mut right = None;
        if v[max_idx+1..v.len()].len() > 0 {
            right = Some(Box::new(Self::new(&v[max_idx+1..v.len()])));
        }

        Self{value: max_val, left, right}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartesian_tree_test() {
        let mut root = CartesianTreeNode::new(&vec![3 ,5 ,2, 1, 4]);
        dbg!(root);
    }
}