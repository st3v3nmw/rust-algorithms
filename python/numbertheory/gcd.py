def gcd(m, n):
    """ O(log(m + n)): Return gcd(m, n) using Euclid's algorithm """
    # abs(m) ensures we don't return a negative number
    return abs(m) if n == 0 else gcd(n, m % n)

def gcd_arr(arr):
    """ O(n): Return the gcd for all elements in the array """
    g = arr[0]
    for i in range(1, len(arr)):
        g = gcd(g, arr[i])
    return g

if __name__ == "__main__":
    print(gcd(0, 0)) # 0
    print(gcd(7, 0)) # 7
    print(gcd(225, 144)) # 9
    print(gcd(144, -225)) # 9
    print(gcd(-144, 225)) # 9
    print(gcd(-144, -225)) # 9
    print(gcd_arr([200, 500, 6000])) # 100