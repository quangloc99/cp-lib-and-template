#include <bits/stdc++.h>
template <class T> int sign(T x) { return x < 0 ? -1 : x > 0; }

#define def_op(type, op)                                                                           \
    type &operator op##=(const type &other) { return *this = *this op other; }                     \
    type operator op(const type &o) const

template <class Num> struct Rational {
    Num num, dem;
    Rational(Num val = 0) : num(val), dem(1) {}
    Rational(Num num_, Num dem_, bool _no_reduce_tag) : num(num_), dem(dem_) {}
    static Rational no_reduce(Num num, Num dem) { return Rational(num, dem, true); }
    Rational(Num num_, Num dem_) : num(num_), dem(dem_) { normalize(); }

    Rational &normalize() {
        Num g = gcd(num, dem);
        num /= g;
        dem /= g;
        normalize_sign();
        return *this;
    }

    Rational &normalize_sign() {
        if (num == 0)
            dem = 1;
        else if (dem < 0) {
            num = -num;
            dem = -dem;
        }
        return *this;
    }

    def_op(Rational, +) { return no_reduce(num * o.dem + o.num * dem, dem * o.dem); }
    def_op(Rational, -) { return no_reduce(num * o.dem - o.num * dem, dem * o.dem); }
    def_op(Rational, *) { return no_reduce(num * o.num, dem * o.dem); }
    def_op(Rational, /) { return no_reduce(num * o.dem, dem * o.num).normalize_sign(); }
    Rational operator-() { return no_reduce(-num, dem); }
    int cmp(Rational other) const { return sign(num * other.dem - other.num * dem); }

    operator long double() const { return (long double)num / (long double)dem; }
    friend std::ostream &operator<<(std::ostream &s, Rational r) { return s << static_cast<long double>(r); }
};

