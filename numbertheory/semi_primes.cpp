#include <bits/stdc++.h>
using namespace std;
#define ll long long

// number of semiprimes in the range 1..n inclusive
ll semi_primes_count(ll n) {
    vector<int> sieve(n+1, 0);
    for (ll i = 2; i <= n; i++) {
        if (sieve[i] == 0) {
            for (ll j = i; j <= n; j += i)
                sieve[j]++;
        }
    }
    
    ll count = 0;
    for (int e : sieve)
        count += e == 2;
    return count;
}

int main() {
    cout << semi_primes_count(1673) << endl;
}