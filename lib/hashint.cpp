#include <bits/stdc++.h>
#ifndef defop
#define defop(type, op)                                                        \
    friend type inline operator op(type u, const type &v) {                    \
        return u op## = v;                                                     \
    }                                                                          \
    type &operator op##=(const type &o)
#endif

template <class U, class V> struct hashint {
    U u;
    V v;
    hashint() : u(), v() {}
    template <class T> hashint(T x) : u(x), v(x) {}
#define defhashop(op)                                                          \
    defop(hashint, op) {                                                       \
        u op## = o.u;                                                          \
        v op## = o.v;                                                          \
        return *this;                                                          \
    }
    defhashop(+) defhashop(-) defhashop(*) defhashop(/);
    auto operator<=>(const hashint &hs) const = default;
    friend std::ostream &operator<<(std::ostream &out, hashint u) { return out << u.u; }
};
