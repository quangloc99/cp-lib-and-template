#include <bits/stdc++.h>

template <class Graph> struct DsuOnTree {
    int n;
    const Graph &gr;

    std::vector<int> subcnt;
    DsuOnTree(int n_, const Graph &gr_) : n(n_), gr(gr_), subcnt(n) {}

    int dfs_cnt(int u, int p) {
        subcnt[u] = 1;
        for (auto v : gr[u]) {
            if (v == p) continue;
            dfs_cnt(v, u);
            subcnt[u] += subcnt[v];
        }
    }

    void solve_dfs(int u, int p) {
        int big_child = -1;
        for (auto v : gr[u]) {
            if (v == p) continue;
            if (big_child == -1 or subcnt[v] > subcnt[big_child]) big_child = v;
        }
        for (auto v : gr[u]) {
            if (v == big_child or v == p) continue;
            solve_dfs(v, u);
            clear_subtree(v, u);
        }
        // update ans?
        if (big_child != -1) { solve_dfs(big_child); }
        // update ans?
        // add_ans ?
        for (auto v : gr[u]) {
            if (v == big_child or v == p) continue;
            // update ans?
            add_subtree(v, u);
        }
    }

    // Fill the belows
    void clear_subtree(int u, int p) {}

    void add_subtree(int u, int p) {}
};
