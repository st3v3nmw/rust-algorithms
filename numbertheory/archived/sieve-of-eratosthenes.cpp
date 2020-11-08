#include <bits/stdc++.h>

using namespace std;
#define ll long long

// O(nlog(log(n))).
vector<int> sieve(int n) {
    vector<bool> is_prime(n+1, true);
    vector<int> primes;
    for (int i = 2; i <= n; i++) {
        if (is_prime[i]) {
            primes.push_back(i);
            for (int j = i * i; j <= n; j += i)
                is_prime[j] = false;
        }
    }
    return primes;
}

int main() {
    vector<int> d = sieve(128);
    for (int e : d)
        cout << e << " ";
    cout << endl;
}