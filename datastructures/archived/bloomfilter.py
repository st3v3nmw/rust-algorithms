import mmh3 # Murmur Hash 3
from math import log, ceil

class BloomFilter:
    def __init__(self, n : int, error_rate : float): # n is size/capacity
        # TODO: check that these computations are correct
        self.m = -n * log(error_rate) / pow(log(2), 2)
        self.k = ceil(self.m / n * log(2))
        self.m = ceil(self.m)

        self.seeds = [i for i in range(self.k)] # function seeds
        self.arr = [0] * self.m
        self.n = n
        self.e = error_rate

    def __repr__(self) -> str:
        return f"n = {self.n}, m = {self.m}, k = {self.k}, e = {self.e}"
    
    def add(self, q : str):
        for seed in self.seeds:
            self.arr[mmh3.hash128(q, seed, signed = False) % self.m] = 1

    def query(self, q : str) -> bool:
        for seed in self.seeds:
            if self.arr[mmh3.hash128(q, seed, signed = False) % self.m] == 0:
                return False # not in the set
        return True # probably in the set


if __name__ == "__main__":
    db = "Nairobi,Munich,Cologne,Paris,Cape Town,Pretoria,Zurich,London,San Francisco,New York,Berlin,Beijing,Seoul,Tokyo,Dubai".split(",")
    bloom = BloomFilter(len(db), 0.01)
    for city in db:
        bloom.add(city)

    test = "Zurich,Cairo,Reykjavik,East London,Kampala,Kyoto,Qatar,Sydney,Nairobi,Rio de Janerio,Moscow,Rabat".split(",")
    for city in test:
        print(f"{city} {bloom.query(city)}")
    print(bloom)

    from random import randint
    db2 = [str(randint(0, 2047)) for _ in range(124)]
    bloom2 = BloomFilter(len(db2), 0.5)
    for t in db2:
        bloom2.add(t)

    test = [str(randint(0, 2047)) for _ in range(256)]
    false_pos = 0
    for t in test:
        if bloom2.query(t) and t not in db2:
            false_pos += 1
    print(bloom2, false_pos / 256)