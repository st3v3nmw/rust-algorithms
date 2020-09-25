#include <bits/stdc++.h>

using namespace std;

vector<uint> suffix_array(const string& s) {
    vector<uint> s_array;
    for (uint i = 0; i < s.size(); i++)
        s_array.push_back(i);

    size_t t = s.size();
    auto cmp = [&s, &t](uint a, uint b) {
        uint l = a > b ? t - a + 1 : t - b + 1;
        for (uint i = 0; i < l; i++) {
            if (s[a + i] > s[b + i])
                return false;
            else if (s[a + i] < s[b + i])
                return true;
        }
        return false;
    };

    sort(s_array.begin(), s_array.end(), cmp);
    return s_array;
}

int main() {

}