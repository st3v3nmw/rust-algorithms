def insertion_sort(arr):
    for i in range(1, len(arr)):
        for j in range(i, 0, -1):
            if arr[j] < arr[j - 1]:
                arr[j], arr[j - 1] = arr[j - 1], arr[j]
            else:
                break
    return arr

if __name__ == "__main__":
    print(insertion_sort([3, -2, 9, 0, 12, -5, 8, 0])) # [-5, -2, 0, 0, 3, 8, 9, 12]