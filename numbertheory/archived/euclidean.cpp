#include <bits/stdc++.h>

using namespace std;

// Euclidean Algorithm GCD
template <typename T>
T gcd(T m, T n) { // Compute the GCD of m & n using Euclid's Algorithm
    if (n == 0) return fabs(m);
    return gcd(n, m % n);
}

template <typename T>
T gcd_arr(const vector<T>& arr) { // Compute the GCD of a vector
    T gcd_r = arr[0];
    for (auto num : arr)
        gcd_r = __gcd(gcd_r, num);
    return gcd_r;
}

// Extended Euclidean Algorithm
int extended_euclidean(int a, int b, int& x, int& y) {
    x = 1, y = 0;
    int x1 = 0, y1 = 1, a1 = a, b1 = b;
    while (b1) {
        int q = a1 / b1;
        tie(x, x1) = make_tuple(x1, x - q * x1);
        tie(y, y1) = make_tuple(y1, y - q * y1);
        tie(a1, b1) = make_tuple(b1, a1 - q * b1);
    }
    return a1;
}

int main() {
    cout << gcd(0, 0) << "\n"; // 0
    cout << gcd(7, 0) << "\n"; // 7
    cout << gcd(225, 144) << "\n"; // 9
    cout << gcd(144, -225) << "\n"; // 9
    cout << gcd(-144, 225) << "\n"; // 9
    cout << gcd(-144, -225) << "\n"; // 9

    cout << gcd_arr(vector<int>{200, 500, 6000}) << endl; // 100
    return 0;
}