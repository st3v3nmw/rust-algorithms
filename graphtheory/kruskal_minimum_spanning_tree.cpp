#include <bits/stdc++.h>

using namespace std;

class union_find {
    public:
        union_find(uint n) {
            parent = vector<uint> (n);
            size = vector<uint> (n, 1);
            for (uint i = 0; i < n; i++)
                parent[i] = i;
        }

        uint find(uint x) {
            if (parent[x] == x)
                return x;
            parent[x] = find(parent[x]);
            return parent[x];
        }

        void unify(uint x, uint y) {
            uint p_x = find(x);
            uint p_y = find(y);

            if (p_x == p_y)
                return; // x and y are already in the same set
            
            parent[p_x] = p_y;
            size[p_y] += size[p_x];
        }

        bool connected(uint x, uint y) {
            return find(x) == find(y);
        }

        vector<uint> parent, size;
};

class Edge {
    public:
        Edge(int weight, int src, int dst) {
            Edge::weight = weight;
            Edge::src = src;
            Edge::dst = dst;
        }

        bool operator< (Edge b) {
            return weight < b.weight;
        }

        int weight, src, dst;
};

class Graph {
    public:
        Graph() {}
        Graph(int n_nodes) {
            n = n_nodes;
        }

        void addEdge(int weight, int src, int dst) {
            Edge e(weight, src, dst);
            edgelist.push_back(e);
            totalEdgeWeight += weight;
        }

        Graph kruskal() {
            Graph mst;
            union_find q(n);
            sort(edgelist.begin(), edgelist.end());
            for (int i = 0; i < edgelist.size(); i++) {
                if (!q.connected(edgelist[i].src, edgelist[i].dst)) {
                    q.unify(edgelist[i].src, edgelist[i].dst);
                    mst.addEdge(edgelist[i].weight, edgelist[i].src, edgelist[i].dst);
                }
            }

            return mst;
        }
    
        int n, totalEdgeWeight = 0;
        vector<Edge> edgelist;
};

int main() {
    Graph g(10);
    vector<pair<int, pair<int, int>>> v = {{0, {8, 9}}, {1, {0, 4}}, {1, {2, 8}}, {1, {4, 5}}, {1, {6, 7}}, {2, {1, 3}}, {2, {2, 9}}, {2, {3, 4}}, {2, {3, 7}}, {4, {0, 3}}, {4, {1, 2}}, {4, {2, 7}}, {4, {6, 8}}, {5, {0, 1}}, {5, {3, 5}}, {6, {6, 8}}, {7, {5, 6}}, {11, {3, 6}}};
    for (pair<int, pair<int, int>> e : v)
        g.addEdge(e.first, e.second.first, e.second.second);

    Graph mst = g.kruskal();
    for (Edge e : mst.edgelist)
        cout << "(" << e.src << ", " << e.dst << "), ";
    cout << "\nTotal Edge Weight " << mst.totalEdgeWeight << endl;
}