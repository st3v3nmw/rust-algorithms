# class Node:
#     def __init__(self, data):
#         self.data = data
#         self.left = None
#         self.right = None

#     def insert(self, data):
#         pass

# class BinaryHeap:
#     def __init__(self):
#         self.root = None

#     def insert(self, val):
#         if self.root == None:
#             self.root = Node(val)
#         else:
#             self.root.insert(val)

# class MinHeap(BinaryHeap):
#     def __init__(self):
#         pass

# class MaxHeap(BinaryHeap):
#     def __init__(self):
#         pass

# # polling, peeking, adding, removing, contains

class MinHeap:
    def __init__(self):
        self.data = []

    def insert(self, val):
        # parent at floor(pos / 2)

        self.data.append(val)
        pos = len(self.data)

        while pos > 1:
            parent = int(pos / 2)
            if self.data[parent-1] > self.data[pos-1]:
                # swap
                self.data[parent-1], self.data[pos-1] = self.data[pos-1], self.data[parent-1]
                pos = parent
            else:
                break

        # print(self.data)
    
    def poll(self):
        # left child at position 2n
        # right child at position 2n+1

        llen = len(self.data)
        self.data[0], self.data[llen-1] = self.data[llen-1], self.data[0]
        val = self.data.pop()
        pos = 1

        while 2 * pos <= llen-2:
            # print(self.data)
            if self.data[2*pos-1] <= self.data[2*pos] and self.data[2*pos-1] <= self.data[pos-1]:
                self.data[pos-1], self.data[2*pos-1] = self.data[2*pos-1], self.data[pos-1]
                pos = 2*pos
            elif self.data[2*pos] < self.data[pos-1]:
                self.data[pos-1], self.data[2*pos] = self.data[2*pos], self.data[pos-1]
                pos = 2*pos+1
            else:
                print("oh, no!")
                break
        # print(self.data)
        return

    def remove(self, val):
        pass


class MaxHeap:
    def __init__(self):
        self.minheap = MinHeap()

    def insert(self, val):
        self.minheap.insert(-1 * val)

    def poll(self):
        return -1 * self.minheap.poll()

if __name__ == "__main__":
    heap = MinHeap()
    heap.insert(3)
    heap.insert(1)
    heap.insert(6)
    heap.insert(5)
    heap.insert(2)
    heap.insert(4)

    heap.poll()

    import heapq
    a = [3, 1, 6, 5, 2, 4]
    heapq.heapify(a)
    heapq.heappop(a)
    print(a)

    from random import randint
    heap = MinHeap()
    a = []
    for _ in range(45):
        r = randint(0, 100)
        heap.insert(r)
        heapq.heappush(a, r)
        k = randint(0, 5)
        if k == 1:
            heap.poll()
            heapq.heappop(a)
        try:
            assert a == heap.data
            print("dzaaaam")
        except AssertionError:
            print(a, heap.data)