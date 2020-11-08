#include <bits/stdc++.h>
using namespace std;
#define ll long long

// O(nlog(n)) - dp & binary search
int longest_inc_subseq(vector<ll> const& arr) {
    const int n = arr.size();
    vector<ll> d(n+1, 1e18);
    d[0] = -1e18;
    for (int i = 0; i < n; i++) {
        int j = upper_bound(d.begin(), d.end(), arr[i]) - d.begin();
        if (d[j-1] < arr[i] && arr[i] < d[j])
            d[j] = arr[i];
    }

    for (ll e : d)
        cout << e << " ";
    cout << "\n";

    int ans = 0;
    for (int i = 0; i <= n; i++) {
        if (d[i] < 1e18)
            ans = i;
        cout << d[i] << endl;
    }
    return ans;
}

// O(n^2) - dp
vector<ll> longest_inc_subseq_quad(vector<ll> const& arr) {
    vector<vector<ll>> arr2(arr.size());
    for (int i = 0; i < arr.size(); i++)
        arr2[i] = {1, i, i};
    for (int i = 1; i < arr.size(); i++) {
        for (int j = 0; j < i; j++) {
            arr2[i][0];
            if (arr[i] > arr[j]) {
                if (arr2[j][0] >= arr2[i][0]) {
                    arr2[i][0] = arr2[j][0] + 1;
                    arr2[i][1] = j;
                }
            }
        }
    }

    // restore subsequence
    vector<ll> l = arr2[0];
    for (vector<ll> e : arr2) {
        if (e[0] > l[0])
            l = e;
    }
    vector<ll> subseq = {arr[l[2]]};
    while (l[1] != l[2]) {
        l = arr2[l[1]];
        subseq.push_back(arr[l[2]]);
    }
    reverse(subseq.begin(), subseq.end());
    return subseq;
}

int main() {
    longest_inc_subseq({0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15});
    // for (ll e : v)
    //     cout << e << " ";
    // cout << longest_inc_subseq({0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15}) << endl;
    // vector<ll> v2 = longest_inc_subseq({6, 2, 5, 1, 7, 4, 8, 3});
    // for (ll e : v2)
    //     cout << e << " ";
    // cout << longest_inc_subseq({6, 2, 5, 1, 7, 4, 8, 3}) << endl;
}