from functools import reduce
from gcd import gcd

def lcm(m, n):
    """ O(log(m + n)): Returns the lcm of m and n """
    # abs(m * n) ensures we don't return a negative number
    return abs(m * n) // gcd(m, n)

def lcm_arr(arr):
    """ O(n): Return the lcm for all elements in the array """
    return reduce(lambda m, n: lcm(m, n), arr)

if __name__ == "__main__":
    print(lcm(144, 225)) # 3600
    print(lcm(144, -225)) # 3600
    print(lcm(-144, 225)) # 3600
    print(lcm(-144, -225)) # 3600
    print(lcm_arr(list(range(1,21)))) # 232792560