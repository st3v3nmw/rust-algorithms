def longest_inc_subseq_quad(arr: list) -> list:
    arr2 = [[1, i, i] for i in range(len(arr))]
    for i in range(1, len(arr)):
        for j in range(i):
            if arr[i] > arr[j]:
                if arr2[j][0] >= arr2[i][0]:
                    arr2[i][0] = arr2[j][0] + 1
                    arr2[i][1] = j
    l = max(arr2, key=lambda x: x[0])
    subseq = [arr[l[2]]]
    while l[1] != l[2]:
        l = arr2[l[1]]
        subseq.append(arr[l[2]])
    
    return list(reversed(subseq))

if __name__ == "__main__":
    print(longest_inc_subseq_quad([0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15]))
    print(longest_inc_subseq_quad([6, 2, 5, 1, 7, 4, 8, 3]))