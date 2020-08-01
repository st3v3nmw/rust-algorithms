#include <iostream>

using namespace std;

unsigned long long factorial(unsigned int n) {
    /* O(n!): Return the factorial */
    if (n == 0)
        return 1;
    return n * factorial(n - 1);
}

int main() {
    cout << factorial(42) << endl;
    return 0;
}