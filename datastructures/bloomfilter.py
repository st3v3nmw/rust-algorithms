import hashlib

class BloomFilter:
    funcs = [hashlib.sha512, hashlib.sha384, hashlib.sha256, hashlib.sha224, hashlib.sha1, hashlib.md5]

    def __init__(self, size : int, k : int):
        if not 1 <= k <= len(self.funcs):
            raise ValueError(f"k should be 1 <= k <= {len(self.funcs)}")

        self.arr = [0] * size
        self.size = size
        self.k = k

    def __repr__(self):
        return str(self.arr)
    
    def add(self, q : str):
        q = q.encode("utf-8")

        for i in range(self.k):
            self.arr[int(self.funcs[i](q).hexdigest(), 16) % self.size] = 1

    def query(self, q : str) -> int:
        q = q.encode("utf-8")
        for i in range(self.k):
            if self.arr[int(self.funcs[i](q).hexdigest(), 16) % self.size] == 0:
                return 0 # not in set
        return 1 # probably in the set


if __name__ == "__main__":
    bloom = BloomFilter(128, 4)
    test = "Nairobi,Munich,Cologne,Paris,Cape Town,Pretoria,Zurich,London,San Francisco,New York,Berlin,Beijing,Seoul,Tokyo,Dubai".split(",")
    for t in test:
        bloom.add(t)

    print(bloom.query("Zurich"))
    print(bloom.query("Cairo"))
    print(bloom.query("Reykjavik"))
    print(bloom.query("East London"))
    print(bloom.query("Kampala"))
    print(bloom.query("Kyoto"))