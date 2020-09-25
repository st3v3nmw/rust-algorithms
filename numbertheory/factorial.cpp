#include <bits/stdc++.h>
#define ull unsigned long long
const ull MOD = 1e9 + 7;

using namespace std;

ull factorial_iterative(uint n) { // overflows for large n
    /* O(n): Return the factorial */
    ull result = 1;
    for (uint i = 2; i <= n; i++)
        result *= i;
    return result;
}

uint factorial_iterative_mod(uint n) {
    ull result = 1;
    for (uint i = 2; i <= n; i++)
        result = (result * i) % MOD;
    return result;
}

uint64_t binpow(uint64_t base, uint64_t exp) {
    ull result = 1;
    while (exp > 0) {
        if (exp & 1)
            result = (__uint128_t) result * base;
        base = (__uint128_t) base * base;
        exp >>= 1;
    }
    return result;
}

// O(nlog(log(n))).
vector<uint> sieve(uint n) {
    vector<bool> is_prime(n+1, true);
    vector<uint> primes;
    for (uint i = 2; i <= n; i++) {
        if (is_prime[i]) {
            primes.push_back(i);
            for (uint j = i * i; j <= n; j += i)
                is_prime[j] = false;
        }
    }
    return primes;
}

ull factorial_legendre(uint n) {
    vector<uint> s = sieve(n);
    ull result = 1;
    for (uint p : s) {
        uint v = 0, n2 = n;
        while (n2) {
            n2 /= p;
            v += n2;
        }
        result = result * binpow(p, v);
    }
    return result;
}

int main() {
    cout << factorial_iterative(42) << endl;
    cout << factorial_legendre(42) << endl;
    cout << factorial_iterative_mod(1000001) << endl;;
    return 0;
}