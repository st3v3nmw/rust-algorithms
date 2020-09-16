#include <bits/stdc++.h>
using namespace std;

vector<int> kmp(string const& s) {
    int s_len = s.size();
    vector<int> pi(s_len, 0);
    for (int i = 1; i < s_len; i++) {
        int j = pi[i-1];
        while (j > 0 && s[i] != s[j])
            j = pi[j-1];
        if (s[i] == s[j])
            j++;
        pi[i] = j;
    }
    return pi;
}

int main() {
    vector<int> pi = kmp("aabaaab");
    for (int i = 0; i < pi.size(); i++)
        cout << pi[i] << " ";
    cout << endl;
}