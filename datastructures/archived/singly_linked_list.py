class Node:
    def __init__(self, data):
        self.data = data
        self.next = None

class SinglyLinkedList:
    def __init__(self):
        self.head = None
        self.tail = None
        self.length = 0
    
    def __len__(self) -> int:
        return self.length

    def __repr__(self) -> str:
        r = ""
        curr = self.head
        while curr is not None:
            r += str(curr.data) + " -> "
            curr = curr.next
        return r + "NULL"

    def insert_head(self, value):
        node = Node(value)
        node.next = self.head
        self.head = node

        if self.tail is None:
            self.tail = self.head
        self.length += 1

    def insert_tail(self, value):
        node = Node(value)
        
        if self.tail is None:
            self.head = node
            self.tail = self.head
        else:
            curr = self.head
            while curr.next != None:
                curr = curr.next
            curr.next = node
            self.tail = curr
        self.length += 1

    def insert_pos(self, value, pos):
        pass

    def remove_head(self):
        if self.head is None:
            raise Exception("Cannot remove the head, list is empty")
        
        val = self.head.data
        if self.head.next is None:
            self.tail = None
            self.head = None
        else:
            self.head = self.head.next
        self.length -= 1
        return val

    def remove_tail(self):
        if self.tail is None:
            raise Exception("Cannot remove the tail, list is empty")

        val = self.tail.data
        if self.tail == self.head:
            # we have 1 node
            self.tail = None
            self.head = None
        else:
            curr = self.head
            while self.tail != curr.next:
                curr = curr.next
            curr.next = None
            self.tail = curr
        self.length -= 1
        return val

    def remove_pos(self, pos):
        pass

    def search(self, item):
        pass

if __name__ == "__main__":
    llist = SinglyLinkedList()
    llist.insert_tail(1) # 1 -> NULL
    llist.insert_head(2) # 2 -> 1 -> NULL
    llist.insert_head(3) # 3 -> 2 -> 1 -> NULL
    llist.remove_tail() # 3 -> 2 -> NULL
    llist.remove_head() # 2 -> NULL
    llist.insert_tail(4) # 2 -> 4 -> NULL
    llist.insert_tail(5) # 2 -> 4 -> 5 -> NULL
    print(llist)