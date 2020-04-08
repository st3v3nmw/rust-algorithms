def linear_search(arr, elem):
    """ O(n): Return the index of the first element found else -1 """
    for i in range(len(arr)):
        if arr[i] == elem:
            return i
    return -1

if __name__ == "__main__":
    print(linear_search([1, -2, 3, 8, 9], -2)) # 1
    print(linear_search([1, -2, 3, 8, 9], 42)) # -1