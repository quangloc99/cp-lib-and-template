#include <bits/stdc++.h>

template <class Elm> struct MatrixPolicy {
    Elm add(Elm u, Elm v) { return u + v; };
    Elm sub(Elm u, Elm v) { return u - v; };
    Elm mul(Elm u, Elm v) { return u * v; }
    bool is_zero(Elm u) { return u == 0; }
    Elm zero() { return 0; }
    Elm one() { return 1; }
};

template <class Elm, class Policy = MatrixPolicy<Elm>> struct Matrix {
    static constexpr Policy P;

    int n, m;
    std::vector<Elm> data;
    Matrix(int n_, int m_) : n(n_), m(m_), data(n * m, P.zero()) {
    }
    Matrix(int n_, int m_, std::initializer_list<Elm> elms)
        : n(n_), m(m_), data(elms) {
        assert(ssize(data) == n * m);
    }

    Elm *operator[](size_t i) {
        return data.data() + i * m;
    }
    const Elm *operator[](size_t i) const {
        return data.data() + i * m;
    }

    // TODO operator+
    // TODO operator-

    friend Matrix operator*(const Matrix &u, const Matrix &v) {
        assert(u.m == v.n);
        Matrix ans(u.n, v.m);
        for (int i = u.n; i--;) {
            for (int f = v.m; f--;) {
                for (int g = u.m; g--;) {
                    ans[i][f] = P.add(ans[i][f], P.mul(u[i][g], v[g][f]));
                }
            }
        }
        return ans;
    }

    static Matrix ident(int n) {
        Matrix ans(n, n);
        while (n--)
            ans[n][n] = P.one();
        return ans;
    }

    Matrix pow(long long exp) const {
        assert(n == m);
        Matrix ans = ident(n), base = *this;
        for (; exp > 0; exp >>= 1, base = base * base)
            if (exp & 1)
                ans = ans * base;
        return ans;
    }

    /**
     * @return determinant _factor_. That is, how _much_ the
     * determinant has changed.
     */
    Elm gausse_elimination() {
        auto &mat = *this;
        Elm det_factor = 1;
        for (int i = 0; i < n; ++i) {
            for (int f = i + 1; f < n; ++f) {
                if (P.is_zero(mat[f][i])) continue;
                for (int g = 0; g < m; ++g) std::swap(mat[f][g], mat[i][g]);
                det_factor = P.mul(det_factor, -1);
                break;
            }
            auto x = mat[i][i];
            if (P.is_zero(x)) continue;
            for (int f = 0; f < n; ++f) {
                if (f == i) continue;
                det_factor *= x;
                auto y = mat[f][i];
                for (int g = 0; g < m; ++g) {
                    mat[f][g] = P.sub(P.mul(mat[f][g], x), P.mul(mat[i][g], y));
                }
            }
        }
        return det_factor;
    }
    
    Elm det() const {
        auto clone = *this;
        auto ans = P.one() / clone.gausse_elimination();
        for (int i = 0; i < std::min(n, m); ++i) ans = P.mul(ans, clone[i][i]);
        return ans;
    }
};
