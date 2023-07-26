#include <bits/stdc++.h>

template <class Num> struct QuadraticPartitionNumber {
    std::vector<std::vector<Num>> values;
    std::vector<Num> partition_values;

    QuadraticPartitionNumber(int n, int cnt_lim = -1) {
        if (cnt_lim == -1)
            cnt_lim = n;
        values.resize(n + 1, vector<Num>(cnt_lim + 1));
        values[0][0] = 1;
        for (int i = 1; i <= cnt_lim; ++i) values[0][i] = 0;

        for (int i = 1; i <= n; ++i) {
            values[i][0] = 0;
            for (int f = i + 1; f <= cnt_lim; ++f) {
                values[i][f] = 0;
            }

            for (int f = 1; f <= std::min(i, cnt_lim); ++f) {
                if (i - f >= 0) {
                    values[i][f] += values[i - f][f];
                    values[i][f] += values[i - 1][f - 1];
                }
            }
        }

        partition_values.resize(n + 1);
        for (int i = 0; i <= n; ++i) {
            partition_values[i] =
                std::accumulate(values[i].begin(), values[i].end(), Num(0));
        }
    }
};

