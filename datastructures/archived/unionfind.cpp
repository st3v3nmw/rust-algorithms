#include <bits/stdc++.h>

using namespace std;

// Has path compression
class union_find {
    public:
        union_find(unsigned int n) {
            parent = vector<unsigned int> (n);
            size = vector<unsigned int> (n, 1);
            for (unsigned int i = 0; i < n; i++)
                parent[i] = i;
        }

        unsigned int find(unsigned int x) {
            if (parent[x] == x)
                return x;
            parent[x] = find(parent[x]);
            return parent[x];
        }

        void unify(unsigned int x, unsigned int y) {
            unsigned int p_x = find(x);
            unsigned int p_y = find(y);

            if (p_x == p_y)
                return; // x and y are already in the same set
            
            parent[p_x] = p_y;
            size[p_y] += size[p_x];
        }

        bool connected(unsigned int x, unsigned int y) {
            return find(x) == find(y);
        }

        vector<unsigned int> parent, size;
};

int main() {
    union_find q(10);
    q.unify(2, 1);
    q.unify(4, 3);
    q.unify(8, 4);
    q.unify(9, 3);
    cout << q.connected(2, 1);
    cout << q.connected(1, 9) << endl;
}