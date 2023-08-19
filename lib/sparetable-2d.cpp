#include <bits/stdc++.h>
#include <bit>
template <class T> struct MaxSpareTable2D {
    std::vector<std::vector<std::vector<std::vector<T>>>> data;

    MaxSpareTable2D(const std::vector<std::vector<T>> &a) {
        size_t n = a.size();
        size_t m = a[0].size();

        size_t ln = std::bit_width(n) + 1;
        size_t lm = std::bit_width(m) + 1;

        data = vector(ln, vector(lm, vector(n, vector<T>(m))));

        data[0][0] = a;
        for (size_t u = 0; u < ln; ++u) {
            for (size_t v = 0; v < lm; ++v) {
                if (u == 0 and v == 0) continue;
                size_t pu = 1 << u;
                size_t pv = 1 << v;
                for (size_t i = 0; i + pu <= n; ++i) {
                    for (size_t f = 0; f + pv <= m; ++f) {
                        if (u) {
                            data[u][v][i][f] =
                                max(data[u - 1][v][i][f], data[u - 1][v][i + pu / 2][f]);
                        } else {
                            data[u][v][i][f] =
                                max(data[u][v - 1][i][f], data[u][v - 1][i][f + pv / 2]);
                        }
                    }
                }
            }
        }
    }

    T get(int r1, int c1, int r2, int c2, const T& default_value = T()) const {
        if (r1 == r2 or c1 == c2) return default_value;
        auto ln = std::bit_width((size_t)(r2 - r1)) - 1;
        auto lm = std::bit_width((size_t)(c2 - c1)) - 1;
        auto res = max({
            data[ln][lm][r1][c1],
            data[ln][lm][r2 - (1 << ln)][c1],
            data[ln][lm][r1][c2 - (1 << lm)],
            data[ln][lm][r2 - (1 << ln)][c2 - (1 << lm)],
        });
        return res;
    }
};
