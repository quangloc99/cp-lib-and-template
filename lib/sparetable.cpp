#include <bits/stdc++.h>
template <class Data> struct SpareTable {
    std::vector<std::vector<Data>> d;

    SpareTable() = default;
    template <class T> SpareTable(const T &arr) {
        d = {vector<Data>(sz(arr))};
        for (int i = (int)arr.size(); i--; ) d[0][i] = arr[i];

        for (int len = 2; len <= sz(arr); len *= 2) {
            auto &cur_layer = d.emplace_back(sz(arr));
            auto &prv_layer = d.rbegin()[1];
            for (int i = 0, f = len / 2; i + len <= sz(arr); ++i, ++f) {
                cur_layer[i] = Data(prv_layer[i], prv_layer[f]);
            }
        }
    }

    Data get(int l, int r) const {
        int lv = std::__lg(r - l);
        return Data(d[lv][l], d[lv][r - (1 << lv)]);
    }
};

struct Data {
    int x;

    Data(int x_ = 0) : x(x_) {}
    Data(const Data& lhs, const Data& rhs): x(std::max(lhs.x, rhs.x)) {}
};

