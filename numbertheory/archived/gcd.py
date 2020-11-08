from functools import reduce

def gcd(m, n):
    """ O(log(m + n)): Return gcd(m, n) using Euclid's algorithm """
    return abs(m) if n == 0 else gcd(n, m % n)

def gcd_arr(arr):
    """ O(n): Return the gcd for all elements in the array """
    return reduce(lambda m, n: gcd(m, n), arr)

if __name__ == "__main__":
    print(gcd(0, 0)) # 0
    print(gcd(7, 0)) # 7
    print(gcd(225, 144)) # 9
    print(gcd(144, -225)) # 9
    print(gcd(-144, 225)) # 9
    print(gcd(-144, -225)) # 9
    print(gcd_arr([200, 500, 6000])) # 100