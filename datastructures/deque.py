from queue_custom import ListQueue

class Deque(ListQueue):
    def __init__(self):
        super().__init__()

    @property
    def back(self):
        if len(self.data) == 0:
            raise Exception("Queue Underflow")
        return self.data[len(self.data) - 1]

    def enqueue_right(self, elem):
        super().enqueue(elem)
    
    def enqueue_left(self, elem):
        self.data.insert(0, elem)
    
    def dequeue_right(self):
        if len(self.data) == 0:
            raise Exception("Queue Underflow")
        return self.data.pop()

    def dequeue_left(self):
        return super().dequeue()

if __name__ == "__main__":
    deque = Deque()
    deque.enqueue_right(8) # [8]
    deque.enqueue_right(12) # [8, 12]
    deque.enqueue_left(7) # [7, 8, 12]
    print(deque.front) # 7
    print(deque.back) # 12
    deque.enqueue_left(42) # [42, 7, 8, 12]
    print(deque)
    print(deque.dequeue_right()) # 12
    print(deque.dequeue_left()) # 42