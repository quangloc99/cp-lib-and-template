#include <bits/stdc++.h>
struct segtree {
    struct Data {
        int x;
        Data(int v = 0): x(v) {}
        Data(const Data& left, Data& right): x(left.x + right.x) {}
    };
    
    int n;
    std::vector<Data> data;
    segtree(int n_): n(n_), data(4 * n) {}
    template<class T>
    segtree(int n_, T values): segtree(n_) {
        build(1, 0, n, values);
    }
    
    template<class T>
    void build(int i, int l, int r, T values) {
        if (r - l == 1) {
            data[i] = values[l];
            return ;
        }
        int mid = (l + r) / 2;
        build(2 * i, l, mid, values);
        build(2 * i + 1, mid, r, values);
        data[i] = Data(data[2 * i], data[2 * i + 1]);
    }
    
    void push(int i, int l, int r) {
        // apply lazy here
        if (r - l > 1) {
            // assign lazy here
        }
        // clear lazy here
    }
    
    void do_point(int pos, int i, int l, int r) {
        /* push(i, l, r); */
        if (pos >= r or l > pos) return ;
        if (r - l == 1) {
            // dot things here
            return ;
        }
        int mid = (l + r) / 2;
        do_point(pos, 2 * i, l, mid);
        do_point(pos, 2 * i + 1, mid, r);
        data[i] = Data(data[2 * i], data[2 * i + 1]);
    }
    
    void do_range(int from, int to, int i, int l, int r) {
        push(i, l, r);
        if (from >= r or l >= to) return ;
        if (from <= l and r <= to) {
            // do things here
            push(i, l, r);
            return ;
        }
        int mid = (l + r) / 2;
        do_range(from, to, 2 * i, l, mid);
        do_range(from, to, 2 * i + 1, mid, r);
        data[i] = Data(data[2 * i], data[2 * i + 1]);
    }
};
