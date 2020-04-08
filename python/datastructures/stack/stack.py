class ListStack:
    """ A stack based on a dynamic array (Python's list) """
    def __init__(self):
        self.data = []

    def __len__(self):
        return len(self.data)
    
    @property
    def top(self):
        llen = len(self.data)
        if llen == 0: return
        return self.data[llen-1]

    def push(self, elem):
        self.data.append(elem)

    def pop(self):
        if len(self.data) == 0: return
        return self.data.pop()

if __name__ == "__main__":
    stack = ListStack()
    stack.push(8) # [8]
    stack.push(12) # [8, 12]
    print(stack.top) # 12
    stack.push(89) # [8, 12, 89]
    stack.pop() # [8, 12]