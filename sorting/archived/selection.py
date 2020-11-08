def selection_sort(arr):
    for i in range(len(arr) - 1):
        idx = i
        for j in range(idx + 1, len(arr)):
            if arr[j] < arr[idx]:
                idx = j
        arr[i], arr[idx] = arr[idx], arr[i]
    return arr

print(selection_sort([3, -2, 9, 0, 12, -5, 8, 0])) # [-5, -2, 0, 0, 3, 8, 9, 12]