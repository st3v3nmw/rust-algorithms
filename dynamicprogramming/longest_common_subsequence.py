from functools import reduce

def longest_common_subseq_iter(string1: str, string2: str) -> str:
    n, m = len(string1), len(string2)
    dp = [[0] * (m + 1) for _ in range(n + 1)]
    for i in range(n):
        for j in range(m):
            if string1[i] == string2[j]:
                dp[i+1][j+1] = 1 + dp[i][j]
            else:
                dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1])

    # get the string
    s = ""
    while m > 0 and n > 0:
        if dp[n][m] == dp[n-1][m]:
            n -= 1
        elif dp[n][m] == dp[n][m-1]:
            m -= 1
        elif dp[n-1][m-1] + 1 == dp[n][m]:
            s = string1[n - 1] + s
            m -= 1
            n -= 1
    return s

def lcs_variadic(*llist) -> str:
    return reduce(longest_common_subseq_iter, llist)

if __name__ == "__main__":
    print(longest_common_subseq_iter("ABABCP", "BABCA"))
    print(longest_common_subseq_iter("ACCGGTCGAGTGCGCGGAAGCCGGCCGAA", "GTCGTTCGGAATGCCGTTGCTCTGTAAA"))
    print(lcs_variadic("ABCBA", "ABABCP", "BABCA"))