def merge_sort(arr):
    """ O(nlog(n)): Returns a sorted array """
    k = int(len(arr) / 2)
    if k == 0: return arr # base case, array length <= 1
    return merge(merge_sort(arr[:k]), merge_sort(arr[k:]))

def merge(arr1, arr2):
    """ O(n): Returns a merged (& sorted) form of arr1 and arr2 """    
    merged = []
    next1, next2 = 0, 0
    while next1 < len(arr1) or next2 < len(arr2):
        if next1 >= len(arr1): # we've reached the end of array1
            merged.extend(arr2[next2:]) # populate with the rest of array2
            break
        elif next2 >= len(arr2): # we've reached the end of array2
            merged.extend(arr1[next1:]) # populate with the rest of array1
            break

        if arr1[next1] < arr2[next2]:
            merged.append(arr1[next1])
            next1+=1
        else:
            merged.append(arr2[next2])
            next2+=1
    return merged

if __name__ == "__main__":
    print(merge_sort([3, -2, -9, 8, 0, -12, 8, 9, 8])) # [-12, -9, -2, 0, 3, 8, 8, 8, 9]