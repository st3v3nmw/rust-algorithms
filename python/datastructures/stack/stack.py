import os
import sys
sys.path.append(os.getcwd())

# yep, weird import
from python.datastructures.linkedlist.singly_linked_list import SinglyLinkedList

class ListStack:
    """ A stack based on a dynamic array (Python's list) """
    def __init__(self):
        self.data = []

    def __len__(self):
        return len(self.data)
    
    @property
    def top(self):
        # peeking
        llen = len(self.data)
        if llen == 0: return
        return self.data[llen-1]

    def push(self, elem):
        self.data.append(elem)

    def pop(self):
        if len(self.data) == 0: return
        return self.data.pop()

    def is_empty(self):
        return len(self.data) == 0

class LinkedStack:
    """ A stack based on a Singly Linked List """
    def __init__(self):
        self.list = SinglyLinkedList()
    
    def __len__(self):
        return self.list.length

    @property
    def top(self):
        if self.list.length == 0: return
        return self.list.head.data

    def push(self, elem):
        self.list.insert_head(elem)
    
    def pop(self):
        if self.list.length == 0: return
        return self.list.remove_head()

if __name__ == "__main__":
    stack = ListStack()
    stack.push(8) # [8]
    stack.push(12) # [8, 12]
    print(stack.top) # 12
    stack.push(89) # [8, 12, 89]
    stack.pop() # [8, 12]

    lstack = LinkedStack()
    lstack.push(8) # 8 -> NULL
    lstack.push(12) # 12 -> 8 -> NULL
    print(lstack.top) # 12
    lstack.push(89) # 89 -> 12 -> 8 -> NULL
    lstack.pop() # 12 -> 8 -> NULL