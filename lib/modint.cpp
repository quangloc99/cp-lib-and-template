#include <bits/stdc++.h>
#ifndef defop
#define defop(type, op)                                                                            \
    friend constexpr type inline operator op(type u, const type &v) { return u op## = v; }         \
    constexpr type &operator op##=(const type &o)
#endif

template <class Mod> struct modint {
    static inline constexpr Mod mod = Mod();
    int x;
    constexpr modint(int y = 0) : x(y % mod()) {}
    constexpr modint(long long y) : x(int(y % mod())) {}
    defop(modint, +) { return ((x += o.x) >= mod() and (x -= mod())), *this; }
    defop(modint, -) { return ((x -= o.x) < 0 and (x += mod())), *this; }
    defop(modint, *) { return *this = modint(1ll * x * o.x); }
    defop(modint, /) { return *this *= o.pow(mod() - 2); }
    modint pow(long long exp) const {
        modint ans = 1, base = *this;
        for (; exp > 0; exp >>= 1, base *= base)
            if (exp & 1) ans *= base;
        return ans;
    }
    auto operator<=>(const modint &other) const = default;
    friend std::ostream &operator<<(std::ostream &out, modint u) { return out << u.x; }
};

using mint = modint<std::integral_constant<int, (int)1e9 + 7>>;

/* int mod; */
/* struct VarMod { */
/*     int operator()() const { return mod; } */
/* }; */
/* using mint = modint<VarMod>; */
