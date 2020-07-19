def binary_search(arr, elem):
    """ O(log(n)): Return True if the element is found else False """
    arr = sorted(arr)
    lower, upper = 0, len(arr) - 1 # initial upper and lower bounds
    while lower <= upper:
        k = int((lower+upper)/2)
        if arr[k] < elem:
            lower = k + 1
        elif arr[k] > elem:
            upper = k - 1
        else:
            return True
    return False

if __name__ == "__main__":
    print(binary_search([1, -2, 3, 8, 9], -2)) # True
    print(binary_search([1, -2, 3, 8, 9], 42)) # False