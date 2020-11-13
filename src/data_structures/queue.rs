#[derive(Debug)]
pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        return Queue { queue: Vec::new() };
    }

    pub fn len(&self) -> usize {
        return self.queue.len();
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    pub fn dequeue(&mut self) -> T {
        return self.queue.remove(0);
    }

    pub fn is_empty(&self) -> bool {
        return self.queue.is_empty();
    }

    pub fn peek(&self) -> Option<&T> {
        return self.queue.first();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_test() {
        let mut q: Queue<i32> = Queue::new();
        q.enqueue(37);
        q.enqueue(42);
        q.enqueue(73);
        q.dequeue();
        assert_eq!(q.dequeue(), 42);
        assert_eq!(*(q.peek().unwrap()), 73);
        assert_eq!(q.len(), 1);
        assert_eq!(q.is_empty(), false);
    }
}