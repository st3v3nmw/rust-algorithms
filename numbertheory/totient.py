def φ(n) -> int:
    """ 
    Euler's totient function φ
    φ(n) is defined as the number of positive integers <= n that are relatively prime to n
    i.e. if n is a prime, then φ(n) = n - 1
    """
    r = m = n
    for i in range(2, n + 1):
        if m % i == 0:
            r *= (1 - 1 / i)
            while m % i == 0:
                m /= i
        if m == 1:
            break
    return int(r)

print(φ(5)) # 4
print(φ(6)) # 2
print(φ(10)) # 4