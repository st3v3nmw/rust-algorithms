#include <bits/stdc++.h>

using namespace std;
#define ll long long

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

int main() {
    vector<uint> d = sieve(128);
    for (uint e : d)
        cout << e << " ";
    cout << endl;
}