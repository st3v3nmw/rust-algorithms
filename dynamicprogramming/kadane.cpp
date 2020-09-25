#include <bits/stdc++.h>

ll kadane(const vector<ll>& v) {
    ll ans = 0, mmax = v[0];
    for (ll e : v) {
        if (e + ans < 0) {
            ans = 0;
            mmax = max(mmax, e);
        } else {
            ans += e;
            mmax = max(mmax, ans);
        }
    }
    return mmax;
}

int main() {
    
}