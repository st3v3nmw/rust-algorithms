#include <bits/stdc++.h>
using namespace std;

vector<uint> kmp(string const& s) {
    uint s_len = s.size();
    vector<uint> pi(s_len, 0);
    for (uint i = 1; i < s_len; i++) {
        uint j = pi[i-1];
        while (j > 0 && s[i] != s[j])
            j = pi[j-1];
        if (s[i] == s[j])
            j++;
        pi[i] = j;
    }
    return pi;
}

int main() {
    vector<uint> pi = kmp("aabaaab");
    for (uint i = 0; i < pi.size(); i++)
        cout << pi[i] << " ";
    cout << endl;
}