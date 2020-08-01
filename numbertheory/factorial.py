def factorial_recursive(n):
    """ O(n!): Return the factorial """
    if n == 0:
        return 1
    return n * factorial_recursive(n - 1)

print(factorial_recursive(42))