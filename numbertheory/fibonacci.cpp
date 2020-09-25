#include <bits/stdc++.h>
using namespace std;

class matrix {
    public:
        matrix(vector<vector<int>> init) {
            for (uint i = 0; i < 2; i++) {
                for (uint j = 0; j < 2; j++)
                    arr[i][j] = init[i][j];
            }
        }

        matrix operator* (matrix other) {
            matrix result({{0, 0}, {0, 0}});
            for (uint i = 0; i < 2; i++) {
                for (uint j = 0; j < 2; j++) {
                    for (uint k = 0; k < 2; k++) {
                        result.arr[i][j] += arr[i][k] * other.arr[k][j];
                    }
                }
            }
            return result;
        }

        array<array<int, 2>, 2> arr; // 2 * 2 matrix
};

matrix binpow(matrix base, uint exp) { // just, fascinating!!!
    matrix result({{1, 0}, {0, 1}}); // doesn't work for f(0)
    while (exp > 0) {
        if (exp & 1)
            result = result * base;
        base = base * base;
        exp >>= 1;
    }
    return result;
}

uint fib_matrix_exp(uint n) { // wrapper
    matrix a({{0, 1}, {1, 1}});
    return binpow(a, n).arr[0][1];
}

int main () {
    for (int i = 0; i <= 40; i++)
        cout << fib_matrix_exp(i) << " ";
    cout << endl;
}