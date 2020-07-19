def quick_sort(arr):
    """ Returns a sorted array """
    k = int(len(arr) / 2)
    if k == 0: return arr
    less_than, pivot, greater_than = [], [], []
    for x in arr:
        if x < arr[k]:
            less_than.append(x)
        elif x > arr[k]:
            greater_than.append(x)
        else:
            pivot.append(x) # stores repeated elements too
    return quick_sort(less_than) + pivot + quick_sort(greater_than)

if __name__ == "__main__":
    print(quick_sort([3, -2, -9, 8, 0, -12, 8, 8])) # [-12, -9, -2, 0, 3, 8, 8, 8]