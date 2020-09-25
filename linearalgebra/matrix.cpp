#include <bits/stdc++.h>
using namespace std;

// 2 * 2 matrix
class matrix {
    public:
        matrix(vector<vector<int>> init) {
            for (uint i = 0; i < 2; i++) {
                for (uint j = 0; j < 2; j++) {
                    arr[i][j] = init[i][j];
                }
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

        array<array<int, 2>, 2> arr;
};

int main () {
    matrix a({{0, 1}, {1, 1}}), b({{0, 1}, {1, 1}});
    matrix c = a * a *a * a * a;
}