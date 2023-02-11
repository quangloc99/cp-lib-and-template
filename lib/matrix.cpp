#include <bits/stdc++.h>
template <class Elm> struct Matrix {
    int n, m;
    std::vector<Elm> data;
    Matrix(int n_, int m_) : n(n_), m(m_), data(n * m) {}
    Matrix(int n_, int m_, std::initializer_list<Elm> elms)
        : n(n_), m(m_), data(elms) {
        assert(ssize(data) == n * m);
    }

    Elm *operator[](size_t i) { return data.data() + i * m; }
    const Elm *operator[](size_t i) const { return data.data() + i * m; }

    // TODO operator+
    // TODO operator-

    friend Matrix operator*(const Matrix &u, const Matrix &v) {
        assert(u.m == v.n);
        Matrix ans(u.n, v.m);
        for (int i = u.n; i--;) {
            for (int f = v.m; f--;) {
                for (int g = u.m; g--;) { ans[i][f] += u[i][g] * v[g][f]; }
            }
        }
        return ans;
    }
    
    static Matrix ident(int n) {
        Matrix ans(n, n);
        while (n--) ans[n][n] = 1;
        return ans;
    }
    
    Matrix pow(long long exp) const {
        assert(n == m);
        Matrix ans = ident(n), base = *this;
        for (; exp > 0; exp >>= 1, base = base * base)
            if (exp & 1) ans = ans * base;
        return ans;
    }
};
