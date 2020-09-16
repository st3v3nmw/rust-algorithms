#include <bits/stdc++.h>

using namespace std;

string longest_common_subseq_iter(const string& string1, const string& string2) {
    int n = string1.size(), m = string2.size();

    vector<vector<int>> dp(n + 1, vector<int>(m + 1));
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            if (string1[i] == string2[j])
                dp[i+1][j+1] = 1 + dp[i][j];
            else
                dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1]);
        }
    }

    string s = "";
    while (m > 0 && n > 0) {
        if (dp[n][m] == dp[n-1][m])
            n--;
        else if (dp[n][m] == dp[n][m-1])
            m--;
        else if (dp[n-1][m-1] + 1 == dp[n][m]) {
            s = string1[n-1] + s;
            m--;
            n--;
        }
    }
    return s;
}

string lcs_variadic(vector<string> args) {
    if (args.size() < 2) {
        cerr << "You should provide at least two strings" << endl;
        exit(0);
    }

    string r = longest_common_subseq_iter(args[0], args[1]);
    for (int i = 2; i < args.size(); i++)
        r = longest_common_subseq_iter(r, args[i]);
    return r;
}

int main() {
    cout << longest_common_subseq_iter("ABABCP", "BABCA") << endl;
    cout << longest_common_subseq_iter("ACCGGTCGAGTGCGCGGAAGCCGGCCGAA", "GTCGTTCGGAATGCCGTTGCTCTGTAAA") << endl;
    cout << lcs_variadic({"ABCBA", "ABABCP", "BABCA"}) << endl;
}