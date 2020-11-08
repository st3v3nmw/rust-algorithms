def bubble_sort(arr):
    """ O(n^2): Returns a sorted array """
    n = len(arr)
    for _ in range(n):
        for i in range(n-1):
            if arr[i] > arr[i+1]:
                arr[i+1], arr[i] = arr[i], arr[i+1] # swap values
    return arr

if __name__ == "__main__":
    print(bubble_sort([3, -2, 9, 0, 0, 12, -5, 8]))