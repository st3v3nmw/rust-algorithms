#include <iostream>
#include <ctime>

inline double get_secs1(timespec m) {
    return m.tv_sec + m.tv_nsec / 1e9;
}

auto get_secs2 = [](timespec m) {
    return m.tv_sec + m.tv_nsec / 1e9;
};

using namespace std;
int main() {
    timespec start, end;
    timespec_get(&start, TIME_UTC);
    // function call goes here
    timespec_get(&end, TIME_UTC);
    cout << get_secs1(end) - get_secs1(start) << endl;
    cout << get_secs2(end) - get_secs2(start) << endl;
    return 0;
}