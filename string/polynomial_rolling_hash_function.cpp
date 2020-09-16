#include <bits/stdc++.h>
using namespace std;
#define ll long long

ll string_hash(string const& s) {
    const int p = 53; // Make p a prime number roughly equal to the number of characters in the input alphabet
    const ll m = 32361122672259149; // now that's a prime (19th Lucas Prime)
    ll result = 0, p_pow = 1;
    for (char c : s) {
        result = (result + (c - 'A' + 1) * p_pow) % m;
        p_pow = (p_pow * p) % m;
    }
    return result;
}

int main() {
    cout << string_hash("bz") << " " << string_hash("book") << endl;
}