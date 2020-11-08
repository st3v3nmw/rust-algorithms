def factorial_iterative(n):
    """ O(n) """
    result = 1;
    for i in range(2, n + 1):
        result *= i
    return result

def factorial_recursive(n):
    if n == 0:
        return 1
    return n * factorial_recursive(n - 1)

print(factorial_recursive(42))
print(factorial_iterative(42))