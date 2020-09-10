""" Suffix Array """

def suffix_array(s : str) -> list:
    arr = [(i, s[i:]) for i in range(len(s))]
    arr.sort(key=lambda x: x[1])
    return [x[0] for x in arr]


""" Longest Common Prefix Array """

def commonPrefixLength(s1 : str, s2 : str) -> int:
    l = min(len(s1), len(s2))
    for i in range(l):
        if s1[i] != s2[i]:
            return i
    return l

def lcp_array(s : str) -> list:
    arr = suffix_array(s)
    lcp = [0]
    for i in range(1, len(s)):
        lcp.append(commonPrefixLength(s[arr[i-1]:], s[arr[i]:]))
    return lcp

""" Counting the number of unique substrings using a Longest Common Prefix Array """

def n_unique_substrs(s : str) -> int:
    n = len(s)
    return n * (n + 1) / 2 - sum(lcp_array(s))

if __name__ == "__main__":
    print(suffix_array("camel"))    # [1, 0, 3, 4, 2]
    print(lcp_array("ABABBAB"))     # [0, 2, 2, 0, 1, 3, 1]
    print(n_unique_substrs("AZAZA"))       # [0, 1, 3, 0, 2]