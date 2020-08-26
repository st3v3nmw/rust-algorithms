class BinaryTreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

    def insert(self, elem):
        if elem <= self.data: # elem less than or equal to this node's value
            if self.left is None:
                self.left = BinaryTreeNode(elem)
            else:
                self.left.insert(elem)
        else:
            if self.right is None:
                self.right = BinaryTreeNode(elem)
            else:
                self.right.insert(elem)
    
    def preorder(self):
        print(self.data, end = " ")
        if self.left is not None:
            self.left.preorder()
        if self.right is not None:
            self.right.preorder()
        
    def inorder(self):
        if self.left is not None:
            self.left.inorder()
        print(self.data, end = " ")
        if self.right is not None:
            self.right.inorder()

    def postorder(self):
        if self.left is not None:
            self.left.postorder()
        if self.right is not None:
            self.right.postorder()
        print(self.data, end = " ")

    def pretty_print(self, x : int, depth : int) -> list:
        llist = []
        # llist.append([x, depth, self.data, self.left is not None, self.right is not None])
        if self.left is not None:
            llist.extend(self.left.pretty_print(x - 1, depth + 1))
        llist.append([x, depth, self.data, self.left is not None, self.right is not None])
        if self.right is not None:
            llist.extend(self.right.pretty_print(x + 1, depth + 1))
        return llist

# class BinaryTree(Tree):
#     def __init__(self):
#         super.__init__()

class BinarySearchTree:
    def __init__(self):
        self.root = None

    def __repr__(self) -> str:
        if self.root is not None: # (x, depth, data)
            nodes = self.root.pretty_print(0, 0)
            print(nodes)
            for i in range(len(nodes)):
                for j in range(i + 1, len(nodes)):
                    if nodes[i][0] == nodes[j][0] and nodes[i][1] == nodes[j][1]:
                        print(nodes[i], nodes[j])
                        # if nodes[i][0] < nodes[j]:
                        #     pass
                        # else:
                        #     pass
                        # for i_ in range(-1, i, -1):
                        #     nodes[i_][0] -= 1
                        for i_ in range(i + 1, len(nodes)):
                            # if nodes[i_][1] >= nodes[i][1]:
                            nodes[i_][0] += 1
            print(nodes)

            minX = min(nodes, key = lambda x : x[0])[0]
            maxD = len(str(max(nodes, key = lambda x : len(str(x[2])))[2]))
            if minX < 0:
                nodes = list(map(lambda x : (x[0] + abs(minX), x[1], x[2], x[3], x[4]), nodes))
            nodes = sorted(nodes, key = lambda x : 10 * x[1] + x[0])

            for i in range(10):
                print(str(i), end="  ")
            print()

            x, y = 0, nodes[0][1]
            linestart = True
            r = []
            k = 0
            for node in nodes:
                if node[1] != y:
                    print("\n" + ''.join(r))
                    r = []
                    x = 0
                    y = node[1]
                    k = 0
                s = str(node[2])
                print(" " * (3 * (node[0] - x) - k) + s, end="")
                # print(s, end=" " * (node[0] - x))
                if node[3]:
                # r += ["  " * node[0] + " / \\"]
                    pass
                if node[4]:
                #     r += " " * len(s) + "\\"
                    pass
                x = node[0]
                k = len(s)
            # print(nodes)
            print()
            print(nodes)
            return "p"
        else:
            return "empty tree"

    def insert(self, elem):
        if self.root is None:
            self.root = BinaryTreeNode(elem)
        else:
            self.root.insert(elem)
    
    def preorder(self):
        if self.root is None:
            return
        self.root.preorder()
        print()
    
    def inorder(self):
        if self.root is None:
            return
        self.root.inorder()
        print()

    def postorder(self):
        if self.root is None:
            return
        self.root.postorder()
        print()

if __name__ == "__main__":
    tree = BinarySearchTree()
    tree.insert(3)
    tree.insert(8)
    tree.insert(1)
    tree.insert(9)
    tree.insert(7)
    tree.insert(2)
    tree.insert(9)
    # tree.insert(8)
    tree.preorder() # 3 -87 8 9 
    tree.inorder() # -87 3 8 9
    tree.postorder() # -87 9 8 3
    print(tree)