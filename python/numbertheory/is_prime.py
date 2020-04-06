from math import sqrt

def is_prime(n):
    """ O(sqrt(n)): Returns True if n is a prime number else False """
    if n < 2: return False

    for i in range(2, int(sqrt(n)) + 1):
        if n % i == 0:
            return False
    return True

if __name__ == "__main__":
    print(is_prime(10)) # False
    print(is_prime(7)) # True
    print(is_prime(2)) # True
    print(is_prime(1)) # False
    print(is_prime(-10)) # False
    print(is_prime(47)) # True