def primes_below_n(n):
	""" Sieve of Eratosthenes """
	sieve = [True] * n
	p = 2
	k = 2
	primes = [2]
	while p < n:
		while k*p < n:
			sieve[k*p] = False
			k += 1
		try:
			p = sieve.index(True, p+1)
			k = 2
			primes.append(p)
		except ValueError:
			break
	return primes


# saw this implementation on ComputerPhile, but Maximum Recursion Depth... :(
# and the version above is about ~89 times faster
# TODO: Write a Haskell version with a "real infinite" list (I'm still a Haskell newbie LOL)
def successor(x):
    x += 1
    yield x
    yield from successor(x)

def sieve(s):
    n = next(s)
    yield n
    yield from sieve(i for i in s if i % n != 0)

s = sieve(successor(1))
llist = []
for _ in range(128):
    llist.append(next(s))
print(llist)

print(primes_below_n(720))