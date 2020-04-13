-- Compute the LCM of m & n
lcmOf m n = fst (abs(m * n) `divMod`(gcd m n))

-- Compute the LCM of a list
lcmArr arr = foldl1 (lcmOf) arr

main = do
    print (lcmOf 144 225) -- 3600
    print (lcmOf 144 (-225)) -- 3600
    print (lcmOf (-144) (225)) -- 3600
    print (lcmOf (-144) (-225)) -- 3600
    print (lcmArr [2..20]) -- 232792560