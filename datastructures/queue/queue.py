import os
import sys
sys.path.append(os.getcwd())

# yep, weird import
from datastructures.linkedlist.singly_linked_list import SinglyLinkedList

class ListQueue:
    """ A queue based on a dynamic array (Python's list) """
    def __init__(self):
        self.data = []

    def __len__(self) -> int:
        return len(self.data)

    def __repr__(self) -> str:
        return str(self.data)
    
    @property
    def front(self):
        # peeking
        if len(self.data) == 0:
            raise Exception("Queue Underflow")
        return self.data[0]

    def enqueue(self, elem):
        self.data.append(elem)

    def dequeue(self):
        if len(self.data) == 0:
            raise Exception("Queue Underflow")
        return self.data.pop(0)

    def is_empty(self) -> bool:
        return len(self.data) == 0

class LinkedQueue:
    """ A queue based on a Singly Linked List """
    def __init__(self):
        self.list = SinglyLinkedList()
    
    def __len__(self) -> int:
        return self.list.length

    def __repr__(self) -> str:
        return str(self.list)

    @property
    def front(self):
        # peeking
        if self.list.length == 0:
            raise Exception("Queue Underflow")
        return self.list.head.data

    def enqueue(self, elem):
        self.list.insert_tail(elem)
    
    def dequeue(self):
        if self.list.length == 0:
            raise Exception("Queue Underflow")
        return self.list.remove_head()
    
    def is_empty(self) -> bool:
        return self.list.length == 0

if __name__ == "__main__":
    queue = ListQueue()
    queue.enqueue(8) # [8]
    queue.enqueue(12) # [8, 12]
    print(queue.front) # 8
    queue.dequeue() # [12]
    queue.enqueue(89) # [12, 89]
    print(queue)

    lqueue = LinkedQueue()
    lqueue.enqueue(8) # 8 -> NULL
    lqueue.enqueue(12) # 8 -> 12 -> NULL
    print(lqueue.front) # 8
    lqueue.dequeue() # 12 -> NULL
    lqueue.enqueue(89) # 12 -> 89 -> NULL
    print(lqueue)