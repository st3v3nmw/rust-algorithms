#include <bits/stdc++.h>

using namespace std;
#define ll long long
#define ull unsigned ll

void factors(int n, map<int, int>& d) {
    int i = 2;
    int upper = floor(sqrt(n)) + 1;
    while(i <= upper) {
        if (n % i == 0) {
            if (d.find(i) == d.end())
                d.insert({i, 1});
            else
                d[i]++;
            n /= i;
        } else 
            i++;
    }

    if (n != 1)
        d.insert({n, 1});
}

int divisors_count(int n) {
    if (n == 1)
        return 1;

    map<int, int> d;
    factors(n, d);
    int result = 1;
    for (pair<int, int> e : d)
        result *= (e.second + 1);
    return result;
}

int divisors_sum(int n) {
    if (n == 1)
        return 1;

    map<int, int> d;
    factors(n, d);
    int result = 1;
    for (pair<int, int> e : d)
        result *= (pow(e.first, e.second + 1) - 1) / (e.first - 1);
    return result;
}

int main() {
    cout << divisors_count(60) << endl; // sexagesimal, 12 divisors
    cout << divisors_sum(60) << endl; // 168
    cout << divisors_sum(53) << endl; // 54
    cout << divisors_count(5) << endl; // 54
}