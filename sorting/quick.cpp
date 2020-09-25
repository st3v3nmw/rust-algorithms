#include <iostream>
#include <vector>

using namespace std;

template <typename T>
vector<T> quick_sort(vector<T> arr) {
    uint p = arr.size() / 2; // pivot position
    vector<T> less_than, pivot, greater_than;

    // partition
    for (auto x : arr) {
        if (x < arr[p])
            less_than.push_back(x);
        else if (x > arr[p])
            greater_than.push_back(x);
        else
            pivot.push_back(x); // store repeated elements too
    }

    // sort subvectors
    if (less_than.size() > 0)
        less_than = quick_sort(less_than);

    if (greater_than.size() > 0)
        greater_than = quick_sort(greater_than);

    // merge into one vector
    arr.clear();
    arr.insert(arr.begin(), less_than.begin(), less_than.end());
    arr.insert(arr.end(), pivot.begin(), pivot.end());
    arr.insert(arr.end(), greater_than.begin(), greater_than.end());
    return arr;
}

int main() {
    vector<int> arr = {3, -2, 9, 0, 0, 12, -5, 8};
    arr = quick_sort(arr); // -5 -2 0 0 3 8 9 12 
    for (uint i = 0; i < arr.size(); i++)
        cout << arr[i] << " ";
    cout << endl;
    return 0;
}