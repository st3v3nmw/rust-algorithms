struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        return Stack { stack: Vec::new() };
    }

    fn len(&self) -> usize {
        return self.stack.len();
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        return self.stack.pop();
    }

    fn is_empty(&self) -> bool {
        return self.stack.is_empty();
    }

    fn peek(&self) -> Option<&T> {
        return self.stack.get(self.stack.len() - 1);
    }
}

fn main() {
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