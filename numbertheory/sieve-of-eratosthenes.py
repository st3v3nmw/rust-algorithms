def sieve(n : int) -> list:
	""" Sieve of Eratosthenes """
	is_prime = [True] * (n + 1)
	primes = []
	for i in range(2, n + 1):
		if (is_prime[i]):
			primes.append(i)
			for j in range(i * i, n + 1, i):
				is_prime[j] = False
	return primes

# saw this implementation on ComputerPhile, but Maximum Recursion Depth... :(
# and the version above is about ~89 times faster
# TODO: Write a Haskell version with a "real infinite" list (I'm still a Haskell newbie LOL)
def successor(x):
    x += 1
    yield x
    yield from successor(x)

def sieve_new(s):
    n = next(s)
    yield n
    yield from sieve_new(i for i in s if i % n != 0)

s = sieve_new(successor(1))
llist = []
for _ in range(128):
    llist.append(next(s))
print(llist)

print(sieve(720))