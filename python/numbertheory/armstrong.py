def armstrong(n):
    """ Return True if n is an Armstrong Number else False """
    if not isinstance(n, int) or n < 1:
        return False
    digits = list(map(int, [x for x in str(n)]))
    n_digits = len(digits)
    return sum([x**n_digits for x in digits]) == n

if __name__ == "__main__":
    print(armstrong(1634)) # True
    print(armstrong(123)) # False
    print(armstrong(5)) # True
    print(armstrong(407)) # True
    print(armstrong(256)) # False