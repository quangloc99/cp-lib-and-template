#include <bits/stdc++.h>
#ifndef defop
#define defop(type, op)                                                                            \
    friend type inline constexpr operator op(type u, const type &v) { return u op## = v; }         \
    constexpr type &operator op##=(const type &o)
#endif
template <int mod> struct modint {
    int x;
    constexpr modint(int y = 0) : x(y) {}
    constexpr modint(long long y) : x(int(y % mod)) {}
    defop(modint, +) { return ((x += o.x) >= mod and (x -= mod)), *this; }
    defop(modint, -) { return ((x -= o.x) < 0 and (x += mod)), *this; }
    defop(modint, *) { return *this = modint(1ll * x * o.x); }
    defop(modint, /) { return *this *= o.pow(mod - 2); }
    constexpr modint pow(long long exp) const {
        modint ans = 1, base = *this;
        for (; exp > 0; exp >>= 1, base *= base)
            if (exp & 1) ans *= base;
        return ans;
    }
    constexpr auto operator<=>(const modint &other) const = default;
    friend std::ostream &operator<<(std::ostream &out, modint u) { return out << u.x; }
};

using mint = modint<998'244'353>;
