from singly_linked_list import SinglyLinkedList

class ListStack:
    """ A stack based on a dynamic array (Python's list) """
    def __init__(self):
        self.data = []

    def __len__(self) -> int:
        return len(self.data)

    def __repr__(self) -> str:
        return str(self.data)
    
    @property
    def top(self):
        # peeking
        llen = len(self.data)
        if llen == 0:
            raise Exception("Stack Underflow")
        return self.data[llen-1]

    def push(self, elem):
        self.data.append(elem)

    def pop(self):
        if len(self.data) == 0:
            raise Exception("Stack Underflow")
        return self.data.pop()

    def is_empty(self) -> bool:
        return len(self.data) == 0

class LinkedStack:
    """ A stack based on a Singly Linked List """
    def __init__(self):
        self.list = SinglyLinkedList()
    
    def __len__(self) -> int:
        return self.list.length

    def __repr__(self) -> str:
        return str(self.list)

    @property
    def top(self):
        if self.list.length == 0:
            raise Exception("Stack Underflow")
        return self.list.head.data

    def push(self, elem):
        self.list.insert_head(elem)
    
    def pop(self):
        if self.list.length == 0:
            raise Exception("Stack Underflow")
        return self.list.remove_head()
    
    def is_empty(self) -> bool:
        return self.list.length == 0

def check_brackets(bracket_string):
    """ Returns True if the brackets sequence is valid else False """
    opening_brackets = ["{", "[", "("]
    closing_brackets = ["}", "]", ")"]
    stack = ListStack()
    for bracket in bracket_string:
        if bracket in opening_brackets:
            stack.push(bracket)
        elif stack.is_empty() or opening_brackets[closing_brackets.index(bracket)] != stack.pop():
            # brackets don't match
            return False
    return stack.is_empty() # valid if the stack is empty

def towers_of_hanoi(n):
    pass


if __name__ == "__main__":
    stack = ListStack()
    stack.push(8) # [8]
    stack.push(12) # [8, 12]
    print(stack.top) # 12
    stack.push(89) # [8, 12, 89]
    stack.pop() # [8, 12]
    print(stack)

    lstack = LinkedStack()
    lstack.push(8) # 8 -> NULL
    lstack.push(12) # 12 -> 8 -> NULL
    print(lstack.top) # 12
    lstack.push(89) # 89 -> 12 -> 8 -> NULL
    lstack.pop() # 12 -> 8 -> NULL
    print(lstack)

    print(check_brackets("[{}]")) # True
    print(check_brackets("(()())")) # True
    print(check_brackets("{]")) # False
    print(check_brackets("[()]))()")) # False
    print(check_brackets("[]{}({})")) # True