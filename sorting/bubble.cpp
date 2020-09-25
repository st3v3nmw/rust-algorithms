#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

template <typename T>
vector<T> bubble_sort(vector<T> arr) {
    // O(n^2): Returns a sorted array
    size_t sz = arr.size();
    T temp;
    for (uint i = 0; i < sz; i++) {
        for (uint j = 0; j < sz - 1; j++) {
            if (arr[j] > arr[j + 1])
                iter_swap(arr.begin() + j, arr.begin() + j + 1);
        }
    }
    return arr;
}

int main() {
    vector<int> arr = {3, -2, 9, 0, 0, 12, -5, 8};
    arr = bubble_sort(arr); // -5 -2 0 0 3 8 9 12 
    for (uint i = 0; i < arr.size(); i++)
        cout << arr[i] << " ";
    cout << endl;
    return 0;
}