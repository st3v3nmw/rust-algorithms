-- Compute the GCD of m & n using Euclid's Algorithm
gcdOf m 0 = abs(m)
gcdOf m n = gcdOf n (mod m n)

-- Compute the GCD of a list
gcdArr arr = foldl1 (gcdOf) arr

main = do
    print (gcdOf 0 0) -- 0
    print (gcdOf 7 0) -- 7
    print (gcdOf 225 144) -- 9
    print (gcdOf 144 (-225)) -- 9
    print (gcdOf (-144) 225) -- 9
    print (gcdOf (-144) (-225)) -- 9
    print (gcdArr [200, 500, 6000]) -- 100