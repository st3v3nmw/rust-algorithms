-- Compute the GCD of m & n using Euclid's Algorithm
gcd' m 0 = abs(m)
gcd' m n = gcd' n (mod m n)

-- Compute the GCD of a list
gcdArr arr = foldl1 (gcd') arr

main = do
    print (gcd' 0 0) -- 0
    print (gcd' 7 0) -- 7
    print (gcd' 225 144) -- 9
    print (gcd' 144 (-225)) -- 9
    print (gcd' (-144) 225) -- 9
    print (gcd' (-144) (-225)) -- 9
    print (gcdArr [200, 500, 6000]) -- 100