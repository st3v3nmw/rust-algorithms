#include <bits/stdc++.h>
using namespace std;
#define ll long long

// number of semiprimes in the range 1..n inclusive
uint semi_primes_count(uint n) {
    vector<uint> sieve(n+1, 0);
    for (uint i = 2; i <= n; i++) {
        if (sieve[i] == 0) {
            for (ll j = i; j <= n; j += i)
                sieve[j]++;
        }
    }
    
    uint count = 0;
    for (uint e : sieve)
        count += e == 2;
    return count;
}

int main() {
    cout << semi_primes_count(1673) << endl;
}