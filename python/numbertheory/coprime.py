from gcd import gcd

def coprime(m, n):
    """ O(log(m + n)): Return True if m and n are relatively prime else False """
    return gcd(m, n) == 1

if __name__ == "__main__":
    print(coprime(3, 7)) # True
    print(coprime(2, 6)) # False
    print(coprime(44, 47)) # True