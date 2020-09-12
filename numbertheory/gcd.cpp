#include <iostream>
#include <cmath>
#include <vector>

using namespace std;

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