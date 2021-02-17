#[derive(Debug)]
struct Node {
    value: i32,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>
}

impl Node {
    fn new(v: &[i32]) -> Self {
        let mut max_idx: usize = 0;
        let mut max_val: i32 = 0;
        for i in 0..v.len() {
            if v[i as usize] > max_val {
                max_val = v[i];
                max_idx = i as usize;
            }
        }
        let mut left = Box::new(None);
        if v[0..max_idx].len() > 0 {
            left = Box::new(Some(Self::new(&v[0..max_idx], table)));
        }

        let mut right = Box::new(None);
        if v[max_idx+1..v.len()].len() > 0 {
            right = Box::new(Some(Self::new(&v[max_idx+1..v.len()], table)));
        }

        Self{value: max_val, left, right}
    }
}

pub struct CartesianTree {
    root: Node
}

pub impl CartesianTree {
    fn new(v: &[i32]) -> Self {
        CartesianTree { root: Node::new(v) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartesian_tree_test() {
        let mut tree = CartesianTree::new(&vec![3 ,5 ,2, 1, 4]);
        dbg!(tree);
    }
}