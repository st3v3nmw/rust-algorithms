#include <bits/stdc++.h>

using namespace std;

class Node {
    public:
        Node(int from, int to) {
            Node::from = from;
            Node::to = to;
            left = nullptr;
            right = nullptr;
        }

        int val, from, to;
        shared_ptr<Node> left, right;
};

class SegmentTree {
    public:
        SegmentTree(vector<int> v) {
            root = build(v, 0, v.size() - 1);
        }

        shared_ptr<Node> build(const vector<int>& v, int from, int to) {
            if (from > to)
                return nullptr;
            shared_ptr<Node> result(new Node(from, to));
            if (from == to)
                result->val = v[from];
            else {
                int m = (from + to) / 2;
                result->left = build(v, from, m);
                result->right = build(v, m + 1, to);
                if (result->left != nullptr)
                    result->val += result->left->val;
                if (result->right != nullptr)
                    result->val += result->right->val;
            }

            return result;
        }

        int query(int from, int to) {
            return query(root, from, to);
        }

        int query(shared_ptr<Node> tree, int from, int to) {
            if (tree == nullptr) return 0;
            if (from <= tree->from && to >= tree->to) return tree->val;
            if (tree->to < from || to < tree->from) return 0;
            return query(tree->left, from, to) + query(tree->right, from, to);
        }

        shared_ptr<Node> root;
};

int main() {
    vector<int> v = {1, 0, 7, 8, 5, 9, 3};
    SegmentTree st(v);
    cout << st.query(0, 6) << endl;
    cout << st.query(0, 5) << endl;
    cout << st.query(2, 5) << endl;
}