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

// TODO: make this efficient across substrings
vector<int> rabin_karp(string const& s, string const& pattern) {
    const ll p_hash = string_hash(pattern);
    const int p_size = pattern.size();
    vector<int> occurences; // indexes of occurences
    for (int i = 0; i < s.size() - p_size; i++) {
        if (string_hash(s.substr(i, p_size)) == p_hash)
            occurences.push_back(i);
    }
    return occurences;
}

int main() {
    vector<int> r = rabin_karp("In computer science, the Rabin–Karp algorithm or Karp–Rabin algorithm is a string-searching algorithm created by Richard M. Karp and Michael O. Rabin (1987) that uses hashing to find an exact match of a pattern string in a text.", "Karp");
    for (int e : r)
        cout << e << " ";
    cout << "\n";
}