#include <bits/stdc++.h>

template <class Data> struct Segtree {
    int n;
    std::vector<Data> data;
    Segtree(int n_) : n(n_), data(4 * n) {}
    template <class T> Segtree(int n_, const T &values) : Segtree(n_) { build(1, 0, n, values); }

    template <class T> void build(int i, int l, int r, const T &values) {
        if (r - l == 1) {
            data[i] = values[l];
            return;
        }
        int mid = (l + r) / 2;
        build(2 * i, l, mid, values);
        build(2 * i + 1, mid, r, values);
        data[i] = Data(data[2 * i], data[2 * i + 1]);
    }

    void push(int i, int l, int r) {
        auto old_lazy = data[i].apply_lazy(l, r);
        if (r - l > 1) {
            data[2 * i].accept_lazy(old_lazy);
            data[2 * i + 1].accept_lazy(old_lazy);
        }
    }

    void update(int from, int to, const auto &lazy, int i, int l, int r) {
        push(i, l, r);
        if (from >= r or l >= to) return;
        if (r - l == 1) {
            data[i].accept_lazy(lazy);
            push(i, l, r);
            return;
        }
        int mid = (l + r) / 2;
        update(from, to, lazy, 2 * i, l, mid);
        update(from, to, lazy, 2 * i + 1, mid, r);
        data[i] = Data(data[2 * i], data[2 * i + 1]);
    }

    void update(int from, int to, const auto &lazy) { update(from, to, lazy, 1, 0, n); }

    void update(int pos, const auto &lazy) { update(pos, pos + 1, lazy); }

    Data query(int from, int to, int i, int l, int r) {
        push(i, l, r);
        if (from >= r or l >= to) return Data();
        if (from <= l and r <= to) { return data[i]; }
        int mid = (l + r) / 2;
        return Data(query(from, to, 2 * i, l, mid), query(from, to, 2 * i + 1, mid, r));
    }

    Data query(int from, int to) { return query(from, to, 1, 0, n); }

    int partition_point(int from, int to, Data &pref_acc, const auto &pred, int i, int l, int r) {
        push(i, l, r);
        int mid = (l + r) / 2;
        if (from >= r or l >= to) return to;
        if (from <= l and r <= to) {
            if (r - l == 1) {
                if (pred(pref_acc, data[i])) return r;
                return l;
            }
            push(2 * i, l, mid);
            if (pred(pref_acc, data[2 * i])) {
                pref_acc = Data(pref_acc, data[2 * i]);
                return partition_point(from, to, pref_acc, pred, 2 * i + 1, mid, r);
            } else {
                return partition_point(from, to, pref_acc, pred, 2 * i, l, mid);
            }
        }
        int x = partition_point(from, to, pref_acc, pred, 2 * i, l, mid);
        if (x == mid)
            return partition_point(from, to, pref_acc, pred, 2 * i + 1, mid, r);
        else
            return x;
    }

    int partition_point(int from, int to, const auto &pred) {
        auto pref_acc = Data();
        return partition_point(from, to, pref_acc, pred, 1, 0, n);
    }
};

struct SumData {
    int sum, lazy;
    SumData(int sum_ = 0) : sum(sum_) {}
    SumData(const SumData &l, const SumData &r) {
        sum = l.sum + r.sum;
        lazy = 0;
    }
    void accept_lazy(int upd) { lazy += upd; }
    int apply_lazy(int l, int r) {
        sum += lazy * (r - l);
        return std::exchange(lazy, 0); // remember to clear lazy
    }
};

void usage() {
    using namespace std;
    ios::sync_with_stdio(0);
    cin.tie(0);
    Segtree<SumData> segtree(10);
    segtree.update(0, 10, 1);
    auto x = segtree.query(0, 10);
    cout << x.sum;
}
