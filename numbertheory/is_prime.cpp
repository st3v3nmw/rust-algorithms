#include <bits/stdc++.h>
using namespace std;
#define ll long long

// O(sqrt(n))
bool trial_division(ll x) {
    for (ll d = 2; d < x; d++) {
        if (x % d == 0)
            return false;
    }
    return true;
}

ll modpow(ll base, ll exp, ll modulus) {
    base %= modulus;
    ll result = 1;
    while (exp > 0) {
        if (exp & 1)
            result = (result * base) % modulus;
        base = (base * base) % modulus;
        exp >>= 1;
    }
    return result;
}

bool fermatProbabilistic(ll n, int iter = 20) {
    if (n < 4)
        return n == 2 || n == 3;
    
    mt19937_64 rng(chrono::steady_clock::now().time_since_epoch().count());
    for (int i = 0; i < iter; i++) {
        ll a = 2 + rng() % (n - 3);
        if (modpow(a, n - 1, n) != 1)
            return false;
    }
    return true;
}

bool rabin_miller(ll n) {
    if (!(n & 1) || n < 2)
        return false;
    // TODO finish this
}

int main() {
    cout << fermatProbabilistic(53) << endl;
    cout << fermatProbabilistic(256) << endl;
    cout << fermatProbabilistic(32361122672259149) << endl; // a Lucas prime, returns false for some reason
    cout << fermatProbabilistic(8911) << endl; // a Carmichael number
}