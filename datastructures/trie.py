class Trie:
    class Node:
        def __init__(self):
            self.children = {}
            self.word_finished = False
            self.is_word = False

    def __init__(self):
        pass