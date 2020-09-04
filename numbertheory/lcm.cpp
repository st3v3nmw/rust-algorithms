#include <iostream>
#include <cmath>
#include <vector>

using namespace std;

template <typename T>
T gcd(T m, T n) {
    if (n == 0) return fabs(m);
    return gcd(n, m % n);
}

auto lcm = []<typename T>(T m, T n) {
    return fabs(m * n) / gcd(m, n);
};

template <typename T>
T lcm_arr(vector<T> arr) {
    T lcm_r = arr[0];
    for (auto num : arr)
        lcm_r = lcm(lcm_r, num);
    return lcm_r;
}

int main() {
    cout << lcm(144, 225) << "\n"; // 3600
    cout << lcm(144, -225) << "\n"; // 3600
    cout << lcm(-144, 225) << "\n"; // 3600
    cout << lcm(-144, -225) << "\n"; // 3600
    
    cout << lcm_arr(vector<long>{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20}) << endl; // 232792560
    return 0;
}