import os
import sys
sys.path.append(os.getcwd())

# yep, weird import
from python.datastructures.stack.stack import ListStack

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

if __name__ == "__main__":
    print(check_brackets("[{}]")) # True
    print(check_brackets("(()())")) # True
    print(check_brackets("{]")) # False
    print(check_brackets("[()]))()")) # False
    print(check_brackets("[]{}({})")) # True