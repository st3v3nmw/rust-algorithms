#include <bits/stdc++.h>
using namespace std;

class Edge {
    public:
        Edge(int src, int dst, int weight) {
            Edge::src = src;
            Edge::dst = dst;
            Edge::weight = weight;
        }

        Edge(int src, int dst) {
            Edge::src = src;
            Edge::dst = dst;
        }

        bool operator< (Edge b) {
            return weight < b.weight;
        }

        int weight, src, dst;
};

class Node {
    public:
        Node(int idx) {
            Node::idx = idx;
        }

        void addEdge(int dst, int weight) {
            Edge e(idx, dst, weight);
            edgelist.push_back(e);
        }

        void addEdge(int dst) {
            Edge e(idx, dst);
            edgelist.push_back(e);
        }

        vector<Edge> edgelist;
        int idx;
};

class Graph {
    public:
        Graph(int n_nodes) {
            for (int i = 0; i < n_nodes; i++)
                nodes.push_back(Node(i));
        }

        void addEdge(int src, int dst, int weight) {
            if (src > nodes.size() - 1 || dst > nodes.size() - 1)
                cout << "Node does not exist\n";
            else
                nodes[src].addEdge(dst, weight);
        }

        void addEdge(int src, int dst) {
            if (src > nodes.size() - 1 || dst > nodes.size() - 1)
                cout << "Node does not exist\n";
            else
                nodes[src].addEdge(dst);
        }

        void dfs() {

        }

        void bfs(int u) {
            if (u > nodes.size() - 1)
                cout << "Node does not exist\n";
            else {
                vector<bool> visited(nodes.size());
                queue<Node> q;
                q.push(nodes[u]);
                while (!q.empty()) {
                    if (visited[q.front().idx]) {
                        q.pop();
                        continue;
                    }

                    Node curr = q.front();
                    q.pop();
                    cout << curr.idx << endl;
                    for (Edge e : curr.edgelist) {
                        if (!visited[e.dst])
                            q.push(nodes[e.dst]);
                    }
                    visited[curr.idx] = true;
                    for (bool q : visited)
                        cout << q;
                    cout << endl;
                }
            }
        }

        vector<Node> nodes;
};

int main() {
    Graph g(10);
    g.addEdge(0, 1);
    // g.addEdge(1, 2);
    g.addEdge(1, 3);
    g.addEdge(3, 5);
    g.addEdge(3, 2);
    g.addEdge(3, 4);
    g.addEdge(4, 5);
    g.addEdge(5, 4);
    g.addEdge(8, 9);
    g.addEdge(2, 9);
    g.addEdge(9, 8);
    g.addEdge(8, 7);
    g.bfs(0);
}