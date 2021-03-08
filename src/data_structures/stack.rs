#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        return Stack { stack: Vec::new() };
    }

    pub fn len(&self) -> usize {
        return self.stack.len();
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.stack.pop();
    }

    pub fn is_empty(&self) -> bool {
        return self.stack.is_empty();
    }

    pub fn peek(&self) -> Option<&T> {
        return self.stack.get(self.stack.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_test() {
        let mut s: Stack<i32> = Stack::new();
        s.push(37);
        s.push(42);
        s.push(73);
        s.pop();
        assert_eq!(s.pop().unwrap(), 42);
        assert_eq!(*(s.peek().unwrap()), 37);
        assert_eq!(s.len(), 1);
        assert_eq!(s.is_empty(), false);
    }
}
