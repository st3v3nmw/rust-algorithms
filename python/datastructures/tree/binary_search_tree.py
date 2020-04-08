class TreeNode:
    def __init__(self, data):
        self.data = data

class BinaryTreeNode(TreeNode):
    def __init__(self, data):
        super.__init__(data)
        self.left = None
        self.right = None

class Tree:
    def __init__(self):
        self.root = None

class BinaryTree(Tree):
    def __init__(self):
        super.__init__()

class BinarySearchTree(BinaryTree):
    def __init__(self):
        super.__init__()