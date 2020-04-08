class ListQueue:
    """ A queue based on a dynamic array (Python's list) """
    def __init__(self):
        self.data = []

    def __len__(self):
        return len(self.data)
    
    @property
    def front(self):
        if len(self.data) == 0: return
        return self.data[0]

    def enqueue(self, elem):
        self.data.append(elem)

    def dequeue(self):
        if len(self.data) == 0: return
        return self.data.pop(0)

if __name__ == "__main__":
    queue = ListQueue()
    queue.enqueue(8) # [8]
    queue.enqueue(12) # [8, 12]
    print(queue.front) # 8
    queue.dequeue() # [12]
    queue.enqueue(89) # [12, 89]