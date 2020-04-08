def bubble_sort(arr):
    """ O(n^2): Return a sorted array """
    n = len(arr)
    for i in range(n):
        for j in range(n-1):
            if arr[j] > arr[j+1]:
                arr[j+1], arr[j] = arr[j], arr[j+1] # swap values
    return arr

if __name__ == "__main__":
    print(bubble_sort([3, -2, 9, 0, 0, 12, -5, 8]))