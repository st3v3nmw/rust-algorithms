#[derive(Debug)]
struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        return Queue { queue: Vec::new() };
    }

    fn len(&self) -> usize {
        return self.queue.len();
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    fn dequeue(&mut self) -> T {
        return self.queue.remove(0);
    }

    fn is_empty(&self) -> bool {
        return self.queue.is_empty();
    }

    fn peek(&self) -> Option<&T> {
        return self.queue.first();
    }
}

fn main() {
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