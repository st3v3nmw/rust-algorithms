-- Iterative Solution
factorialIterative :: Integer -> Integer
factorialIterative n = product [1..n]

-- Recursive solution
factorialRecursive :: Integer -> Integer
factorialRecursive 0 = 1
factorialRecursive n = n * factorialRecursive(n-1)

main = do
    print (factorialIterative 10)
    print (factorialRecursive 10)