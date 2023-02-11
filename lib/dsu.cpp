#include <bits/stdc++.h>
struct DSU {
    std::vector<int> par;
    DSU(int n): par(n, -1) {}
    
    int find_set(int u) {
        return par[u] < 0 ? u : par[u] = find_set(par[u]);
    }
    
    bool join(int u, int v) {
        u = find_set(u);
        v = find_set(v);
        if (u == v) return false;
        if (-par[u] < -par[v]) std::swap(u, v);
        par[u] += par[v];
        par[v] = u;
        return true;
    }
    
    int set_size(int u) {
        return -par[find_set(u)];
    }
};
