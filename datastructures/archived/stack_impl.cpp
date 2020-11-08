#include <bits/stdc++.h>
using namespace std;

inline void move_disk(uint n, stack<uint>& from, stack<uint>& to, stack<uint>& aux) {
    if (n > 0) {
        move_disk(n - 1, from, aux, to);
        to.push(from.top());
        from.pop();
        move_disk(n - 1, aux, to, from);
    }
}

void hanoi(uint n) {
    stack<uint> s1, s2, s3;
    for (uint i = n; i > 0; i--)
        s1.push(i);
    move_disk(n, s1, s3, s2);
}

int main() {
    hanoi(3);
}