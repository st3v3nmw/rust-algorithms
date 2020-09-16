#include <bits/stdc++.h>

using namespace std;
#define ll long long

// O(nlog(log(n))).
vector<ll> sieve(ll n) {
    vector<bool> is_prime(n+1, true);
    vector<ll> primes;
    for (ll i = 2; i <= n; i++) {
        if (is_prime[i]) {
            primes.push_back(i);
            for (ll j = i * i; j <= n; j += i)
                is_prime[j] = false;
        }
    }
    return primes;
}

int main() {
    vector<ll> d = sieve(10000000);
    for (ll e : d)
        cout << e << " ";
    cout << endl;
}