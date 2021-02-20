#[derive(Debug)]
struct BSTNode {
    value: i64,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>
}

impl BSTNode {
    fn new(value: i64) -> Self {
        Self{value, left: None, right: None}
    }

    fn insert(&mut self, value: i64) {
        if value <= self.value {
            match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(Self::new(value)))
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(Self::new(value)))
            }
        }
    }

    fn find(&mut self, value: i64) -> bool {
        if value == self.value {
            return true;
        }

        if value < self.value {
            match self.left {
                Some(ref mut node) => node.find(value),
                None => false
            }
        } else {
            match self.right {
                Some(ref mut node) => node.find(value),
                None => false
            }
        }
    }

    fn traverse_inorder(&mut self) -> Vec<i64> {
        let mut result: Vec<i64> = vec![];
        match self.left {
            Some(ref mut node) => result.extend(node.traverse_inorder()),
            None => ()
        }
        result.push(self.value);
        match self.right {
            Some(ref mut node) => result.extend(node.traverse_inorder()),
            None => ()
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_tree_test() {
        let mut tree = BSTNode::new(5);
        for val in [4,6,8,2,-1,0,3].iter() {
            tree.insert(*val);
        }
        assert_eq!(tree.traverse_inorder(), vec![-1,0,2,3,4,5,6,8]);
    }
}