-- Compute the LCM of m & n
-- lcm = (m * n) / gcd(m, n)
lcm' m n = fst (abs(m * n) `divMod`(gcd m n))

-- Compute the LCM of a list
lcmArr arr = foldl1 (lcm') arr

main = do
    print (lcm' 144 225) -- 3600
    print (lcm' 144 (-225)) -- 3600
    print (lcm' (-144) (225)) -- 3600
    print (lcm' (-144) (-225)) -- 3600
    print (lcmArr [2..20]) -- 232792560